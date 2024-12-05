import json
import matplotlib.pyplot as plt
import matplotlib.patches as patches
import random
import sys
from matplotlib.patches import Polygon

def generate_distinct_colors(n):
    colors = []
    for i in range(n):
        hue = i / n
        saturation = 0.7 + random.random() * 0.3  # 0.7-1.0
        value = 0.8 + random.random() * 0.2       # 0.8-1.0
        colors.append(plt.cm.hsv(hue))
    return colors

def plot_rectangles_from_json(json_path):
    with open(json_path, 'r') as file:
        rectangles = json.load(file)

    plt.rcParams['figure.figsize'] = [19.2, 10.8]  # 1920x1080 pixels at 100 DPI
    fig = plt.figure()

    ax = fig.add_subplot(111)

    colors = generate_distinct_colors(len(rectangles))
    legend_data = []

    all_points = [(point["x"], point["y"]) for rect in rectangles for point in rect["points"]]
    x_coords, y_coords = zip(*all_points)
    x_min, x_max = min(x_coords), max(x_coords)
    y_min, y_max = min(y_coords), max(y_coords)

    padding = 0.1 * max(x_max - x_min, y_max - y_min)
    ax.set_xlim(x_min - padding, x_max + padding)
    ax.set_ylim(y_min - padding, y_max + padding)

    for idx, rect in enumerate(rectangles):
        color = colors[idx]
        points = [(point["x"], point["y"]) for point in rect["points"]]
        is_intersection = any(prop.get("IsIntersection", False) for prop in rect["properties"])

        if is_intersection:
            polygon = Polygon(points, closed=True, linewidth=2, edgecolor=color,
                              facecolor='none', linestyle="--", alpha=0.8)
        else:
            polygon = Polygon(points, closed=True, linewidth=2, edgecolor=color,
                              facecolor=color, alpha=0.3)
        ax.add_patch(polygon)

        for point in rect["points"]:
            px, py = point["x"], point["y"]
            name = point["name"]
            ax.plot(px, py, 'o', color=color, markersize=8, zorder=5)
            ax.text(px, py, f"{name}", ha="center", va="bottom", fontsize=11,
                    color='black', fontweight='bold', bbox=dict(facecolor='white',
                                                                alpha=0.7, edgecolor='none', pad=1), zorder=6)

        center_x = sum(p[0] for p in points) / len(points)
        center_y = sum(p[1] for p in points) / len(points)
        ax.text(center_x, center_y, rect["name"], ha="center", va="center",
                fontsize=14, color='black', fontweight='bold',
                bbox=dict(facecolor='white', alpha=0.7, edgecolor='none', pad=2))

        diagonal = next((prop["Diagonal"] for prop in rect["properties"] if "Diagonal" in prop), None)
        if diagonal:
            ax.plot([points[0][0], points[2][0]], [points[0][1], points[2][1]],
                    color=color, linestyle="--", linewidth=2, alpha=1)

        if rect["properties"]:
            properties_text = ', '.join(
                f"{list(prop.keys())[0]}: {round(float(list(prop.values())[0]), 2)}"
                for prop in rect["properties"]
            )
            legend_data.append((f"{rect['name']} - {properties_text}", color))

    legend_handles = [patches.Patch(color=col, label=text, alpha=0.5) for text, col in legend_data]
    ax.legend(handles=legend_handles, title="Rectangle Properties",
              loc="center left", bbox_to_anchor=(1, 0.5),
              borderaxespad=0., framealpha=0.8, fontsize=10)

    ax.grid(True, linestyle='--', alpha=0.3)
    ax.set_aspect('equal', adjustable='box')

    plt.xlabel("X", fontsize=12, fontweight='bold')
    plt.ylabel("Y", fontsize=12, fontweight='bold')
    plt.title("Shvachka Denys Systemne Lab 2", fontsize=16, fontweight='bold', pad=20)

    ax.tick_params(axis='both', which='major', labelsize=10)

    plt.tight_layout()

    plt.show()

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python script.py <path_to_json>")
        sys.exit(1)

    json_path = sys.argv[1]
    plot_rectangles_from_json(json_path)