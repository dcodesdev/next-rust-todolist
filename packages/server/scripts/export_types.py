import os


def create_index_ts(directory):
    """
    This function creates an index.ts file inside the bindings directory
    if it doesn't exist, otherwise it will delete it and creates a new one
    with all the other typescript types exported.
    """
    # List all .ts files in the directory except 'index.ts'
    files = [f for f in os.listdir(directory) if os.path.isfile(
        os.path.join(directory, f)) and f != 'index.ts' and f.endswith('.ts')]

    # Path to the index.ts file
    index_ts_path = os.path.join(directory, 'index.ts')

    # Delete the old version of index.ts if it exists
    if os.path.exists(index_ts_path):
        os.remove(index_ts_path)

    # Create a new index.ts and write export statements for each .ts file
    with open(index_ts_path, 'w') as index_ts:
        for file in files:
            # Remove the '.ts' extension and prepare the export statement
            module_name = os.path.splitext(file)[0]
            export_statement = f'export * from "./{module_name}"\n'
            index_ts.write(export_statement)

    print(f"Exported {len(files)} files successfully.")

directory = os.path.join(os.path.dirname(
    os.path.abspath(__file__)), "..", "bindings")

create_index_ts(directory)
