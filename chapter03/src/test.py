from x.py import options

x = [1, 2, 3, 5, 45, 5, 2, 3, 4, 5, 6, 6]

options.parser.add_argument(
    "--input_image_1",
    type=str,
    required=True,
)
