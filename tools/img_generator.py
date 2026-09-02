from PIL        import Image
from sys        import exit
from os.path    import exists

def create_output_folder(path: str) -> str:
    from os      import mkdir

    if exists(path):
        return path

    try:
        mkdir(path)

    except Exception as e:
        print(f"'create_output_folder' error: {e}")
        exit(1)
    
    return path 

def generate_images(count: int, path: str) -> None:
    image = Image.new("RGB", (800, 600), (0, 0, 0))

    for i in range(1, count + 1):
        image.save(f"{path}/image_{i}.png")
        print(f"'generate_images' saved: {path}/image_{i}.png")

    print("'generate_images' success.")

def generate_html(image_count: int, path: str) -> None:
    image_elements: list[str] = []
    for i in range(1, image_count + 1):
        image_elements.append(
            f"<img src='generated/image_{i}.png' alt='image number: {i}'>"
        )

    TEMPLATE = f"""
<!DOCTYPE html>
    <html lang="en">

    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Marie Test Server</title>
    </head>

    <body style="background-color: #2f3136; color: white">
        <h1>
            <code>Static Website</code>
            {"\n".join(image_elements)}
        </h1>
    </body>
</html>
"""


    try:
        with open(path, mode = "w") as f:
            f.write(TEMPLATE)

    except Exception as e:
        print(f"'generate_html': error {e}")
        exit(1)

    print("'generate_html': success.")

def main() -> None:
    base_path = "test_server/statics"
    if not exists(base_path):
        print(f"'main' error: {base_path} does not exists")
        exit(1)

    output_path = create_output_folder(f"{base_path}/generated")

    generate_images(10, output_path)
    generate_html(10, f"{base_path}/index.html")

main()