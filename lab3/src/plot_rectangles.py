import json
import matplotlib.pyplot as plt
import matplotlib.patches as patches
import numpy as np
from matplotlib.patches import Rectangle, Circle
from matplotlib.widgets import Button

class RectangleVisualizer:
    def __init__(self, json_path):
        with open(json_path, 'r') as file:
            self.steps = json.load(file)

        self.current_traits = set()
        # Get final rectangle dimensions for reference
        final_rect = self.steps[-1][1]
        self.reference_width = final_rect['width']
        self.reference_height = final_rect['height']

        self.current_step = 0
        self.fig, self.ax = plt.subplots(figsize=(12, 8))
        self.setup_buttons()
        self.draw_current_step()


    def get_rectangle_color(self, traits):
        # Base color in RGB
        base_color = np.array([0.8, 0.8, 1.0])  # Light blue

        if 'Area' in traits:
            # Add red tint
            base_color[0] = min(1.0, base_color[0] + 0.2)
            base_color[2] = max(0.7, base_color[2] - 0.1)

        if 'Perimeter' in traits:
            # Add green tint
            base_color[1] = min(1.0, base_color[1] + 0.2)
            base_color[2] = max(0.7, base_color[2] - 0.1)

        return base_color

    def draw_sides(self, width, height, traits):
        # Draw sides with different colors based on traits
        if 'SmallerSide' in traits or 'BiggerSide' in traits:
            is_width_bigger = width >= height

            # Bottom line
            color_bottom = 'red' if 'BiggerSide' in traits and is_width_bigger else 'green' if 'SmallerSide' in traits and not is_width_bigger else 'blue'
            self.ax.plot([0, width], [0, 0], color=color_bottom, alpha=0.6, linewidth=2)

            # Right line
            color_right = 'red' if 'BiggerSide' in traits and not is_width_bigger else 'green' if 'SmallerSide' in traits and is_width_bigger else 'blue'
            self.ax.plot([width, width], [0, height], color=color_right, alpha=0.6, linewidth=2)

            # Top line
            color_top = 'red' if 'BiggerSide' in traits and is_width_bigger else 'green' if 'SmallerSide' in traits and not is_width_bigger else 'blue'
            self.ax.plot([0, width], [height, height], color=color_top, alpha=0.6, linewidth=2)

            # Left line
            color_left = 'red' if 'BiggerSide' in traits and not is_width_bigger else 'green' if 'SmallerSide' in traits and is_width_bigger else 'blue'
            self.ax.plot([0, 0], [0, height], color=color_left, alpha=0.6, linewidth=2)
        else:
            # Draw regular blue rectangle outline
            self.ax.plot([0, width, width, 0, 0], [0, 0, height, height, 0], 'blue', alpha=0.6, linewidth=2)

    def get_human_readable_trait(self, trait_key):
        trait_mapping = {
            'Perimeter': 'Perimeter',
            'Area': 'Area',
            'Diagonal': 'Diagonal',
            'DiagonalDiagonalAngle': 'Diagonal Angle',
            'SideXDiagonalAngle': 'Side-X Angle',
            'SideYDiagonalAngle': 'Side-Y Angle',
            'SmallerSide': 'Smaller Side',
            'BiggerSide': 'Bigger Side',
            'Ratio': 'Ratio',
            'SideDistances': 'Side Distances',
            'CircumscribedCircleRadius': 'Circle Radius',
            'CircumscribedCircleDiameter': 'Circle Diameter',
            'CircumscribedCircleArea': 'Circle Area',
            'CircumscribedCirclePerimeter': 'Circle Perimeter',
            'CircleRectRatio': 'Circle-Rectangle Ratio'
        }
        return trait_mapping.get(trait_key, trait_key)

    def setup_buttons(self):
        plt.subplots_adjust(bottom=0.2)
        ax_prev = plt.axes([0.3, 0.05, 0.1, 0.075])
        ax_next = plt.axes([0.6, 0.05, 0.1, 0.075])
        self.btn_prev = Button(ax_prev, 'Previous')
        self.btn_next = Button(ax_next, 'Next')
        self.btn_prev.on_clicked(self.previous_step)
        self.btn_next.on_clicked(self.next_step)

    def previous_step(self, event):
        if self.current_step > 0:
            self.current_step -= 1
            self.draw_current_step()

    def next_step(self, event):
        if self.current_step < len(self.steps) - 1:
            self.current_step += 1
            self.draw_current_step()

    def draw_rectangle_corners(self, width, height):
        corners = [
            ('A', (0, 0)),
            ('B', (width, 0)),
            ('C', (width, height)),
            ('D', (0, height))
        ]

        for label, (x, y) in corners:
            self.ax.plot(x, y, 'ko', markersize=8)
            self.ax.annotate(label, (x, y),
                             textcoords="offset points",
                             xytext=(5, 5),
                             fontsize=12,
                             fontweight='bold')

    def draw_side_dimensions(self, width, height):
        # Draw width dimension
        self.ax.annotate(f'{width:.2f}',
                         xy=(width/2, -height*0.015),
                         xytext=(0, -10),
                         textcoords='offset points',
                         ha='center',
                         va='top')

        # Draw height dimension
        self.ax.annotate(f'{height:.2f}',
                         xy=(-width*0.015, height/2),
                         xytext=(-10, 0),
                         textcoords='offset points',
                         ha='right',
                         va='center',
                         rotation=90)

    def draw_side_distances(self, width, height, traits):
        if 'SideDistances' in traits:
            center_x, center_y = width / 2, height / 2
            self.ax.plot([center_x, 0], [center_y, center_y], 'k--', alpha=0.7, label='Side Distance X')
            self.ax.plot([center_x, center_x], [center_y, 0], 'k--', alpha=0.7, label='Side Distance Y')

            side_distances = self.get_trait_value(traits['SideDistances'])
            if isinstance(side_distances, (list, tuple)):
                self.ax.annotate(f'{side_distances[0]:.2f}', xy=(center_x / 2, center_y), ha='center', va='bottom')
                self.ax.annotate(f'{side_distances[1]:.2f}', xy=(center_x, center_y / 2), ha='left', va='center')

    def draw_diagonals(self, width, height, traits):
        diagonal_ac = np.sqrt(width**2 + height**2)
        self.ax.plot([0, width], [0, height], 'r--', alpha=0.7, linewidth=2, label='Diagonal AC')
        self.ax.plot([width, 0], [0, height], 'r--', alpha=0.7, linewidth=2, label='Diagonal BD')

        if 'Diagonal' in traits:
            self.ax.annotate(f'{diagonal_ac:.2f}', xy=(width / 2, height / 3.5), color='red', ha='center', va='bottom')

    def draw_circumscribed_circle(self, width, height, radius=None, diameter=None):
        center_x = width / 2
        center_y = height / 2

        # Calculate radius if only diameter is provided
        if radius is None and diameter is not None:
            radius = diameter / 2

        if radius is not None:
            # Draw the circle
            circle = Circle((center_x, center_y), radius, fill=False,
                            linestyle='--', color='green', alpha=0.7,
                            linewidth=2, label='Circumscribed Circle')
            self.ax.add_patch(circle)

            # Draw radius line from center to top of circle
            self.ax.plot([center_x, center_x], [center_y, center_y + radius],
                         'g-', linewidth=1.5, linestyle=':')

            # Add radius text with a slight offset to the left of the line
            text_x = center_x - radius * 0.1  # Offset to the left
            text_y = center_y + radius / 1.75     # Halfway up the radius line
            self.ax.text(text_x, text_y, f'R={radius:.2f}',
                         rotation=90,
                         verticalalignment='center',
                         horizontalalignment='right',
                         color='green',
                         fontsize=10)

            # Draw diameter line if diagonal trait exists
            if diameter is not None and 'Diagonal' in self.steps[self.current_step][1]['traits']:
                # Draw diameter line from bottom to top of circle
                self.ax.plot([center_x, center_x], [center_y - radius, center_y + radius],
                             'g-', linewidth=1.5, linestyle=':')

                # Add diameter text with a slight offset to the left of the line
                text_x = center_x - radius * 0.1  # More offset to the left than radius text
                text_y = center_y - radius / 1.75  # Middle of the diameter line
                self.ax.text(text_x, text_y, f'D={diameter:.2f}',
                             rotation=90,
                             verticalalignment='center',
                             horizontalalignment='right',
                             color='green',
                             fontsize=10)
    def get_trait_value(self, trait_value):
        if isinstance(trait_value, (int, float)):
            return float(trait_value)
        elif isinstance(trait_value, dict):
            if 'Single' in trait_value:
                return float(trait_value['Single'])
            elif 'Pair' in trait_value:
                return trait_value['Pair']
        return trait_value

    def format_trait_for_legend(self, key, value):
        trait_value = self.get_trait_value(value)
        human_readable_key = self.get_human_readable_trait(key)
        if isinstance(trait_value, (int, float)):
            return f"{human_readable_key}: {trait_value:.2f}"
        elif isinstance(trait_value, (list, tuple)):
            return f"{human_readable_key}: ({trait_value[0]:.2f}, {trait_value[1]:.2f})"
        return f"{human_readable_key}: {trait_value}"

    def calculate_plot_bounds(self, width, height, traits):
        # Start with basic rectangle bounds
        bounds = max(width, height)

        # Check if we have a circumscribed circle
        if 'CircumscribedCircleRadius' in traits:
            radius = self.get_trait_value(traits['CircumscribedCircleRadius'])
            if isinstance(radius, (int, float)):
                # Use circle diameter as the bound
                bounds = max(bounds, radius * 2)

        # Add margin (30% of the bounds)
        margin = bounds * 0.3
        return bounds + margin


    def get_previous_traits(self):
        if self.current_step == 0:
            return set()
        return set(self.steps[self.current_step - 1][1]['traits'].keys())

    def draw_current_step(self):
        self.ax.clear()

        step_description, rect_data = self.steps[self.current_step]

        # Use reference dimensions if current dimensions are 0
        width = rect_data['width'] if rect_data['width'] != 0 else self.reference_width
        height = rect_data['height'] if rect_data['height'] != 0 else self.reference_height
        traits = rect_data['traits']
        previous_traits = self.get_previous_traits()
        current_traits = set(traits.keys())
        self.current_traits = current_traits - previous_traits  # New traits in this step

        rect_color = self.get_rectangle_color(traits)
        # Draw base rectangle
        rect = Rectangle((0, 0), width, height,
                         facecolor=rect_color,
                         alpha=0.5,
                         label='Rectangle')
        self.ax.add_patch(rect)

        self.draw_sides(width, height, traits)
        # Draw corner points and labels
        self.draw_rectangle_corners(width, height)

        # Draw dimensions if they're defined
        if rect_data['width'] != 0 and rect_data['height'] != 0:
            self.draw_side_dimensions(width, height)

        # Add visual elements based on traits

        if 'SideDistances' in traits:
            self.draw_side_distances(width, height, traits)

        if 'Diagonal' in traits:
            self.draw_diagonals(width, height, traits)


        radius = None
        diameter = None
        if 'CircumscribedCircleRadius' in traits:
            radius = self.get_trait_value(traits['CircumscribedCircleRadius'])
        if 'CircumscribedCircleDiameter' in traits:
            diameter = self.get_trait_value(traits['CircumscribedCircleDiameter'])

        if radius is not None or diameter is not None:
            self.draw_circumscribed_circle(width, height, radius, diameter)

        # Prepare legend entries for traits
        legend_entries = []
        legend_labels = []
        for k, v in traits.items():
            formatted_trait = self.format_trait_for_legend(k, v)
            color = 'red' if k in self.current_traits else 'black'
            # Create an invisible patch for each trait
            legend_entries.append(patches.Patch(color='none', alpha=0))
            # Store the colored text
            legend_labels.append(formatted_trait)
            # Apply color to the text
            legend_entries[-1].set_label(formatted_trait)

        handles, labels = self.ax.get_legend_handles_labels()
        handles.extend(legend_entries)

        if handles:
            legend = self.ax.legend(handles=handles,
                                    title='Properties',
                                    loc='center left',
                                    bbox_to_anchor=(1.05, 0.5))

            # Color the text in the legend
            for i, text in enumerate(legend.get_texts()):
                if i >= len(handles) - len(traits):  # Only color the trait entries
                    trait_key = list(traits.keys())[i - (len(handles) - len(traits))]
                    text.set_color('red' if trait_key in self.current_traits else 'black')
        # Set title and adjust display
        self.ax.set_title(f"Step {self.current_step + 1}: {step_description}",
                          pad=20, fontsize=14, fontweight='bold')
        self.ax.set_aspect('equal')
        self.ax.grid(True, linestyle='--', alpha=0.3)

        # Calculate and set bounds to accommodate all elements
        plot_bounds = self.calculate_plot_bounds(width, height, traits)
        self.ax.set_xlim(-plot_bounds * 0.3, plot_bounds * 1.2)
        self.ax.set_ylim(-plot_bounds * 0.3, plot_bounds * 1.2)

        # Add axis labels
        self.ax.set_xlabel('Width', fontsize=12)
        self.ax.set_ylabel('Height', fontsize=12)

        plt.tight_layout()
        plt.draw()

def visualize_rectangle_steps(json_path):
    visualizer = RectangleVisualizer(json_path)
    plt.show()

if __name__ == "__main__":
    import sys
    if len(sys.argv) != 2:
        print("Usage: python script.py <path_to_json>")
        sys.exit(1)

    visualize_rectangle_steps(sys.argv[1])