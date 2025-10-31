# Takes all scripts in the ./scripts directory and makes them linux compatible
# By default they are setup to run with bash on Windows

# Makes the following changes, and places an identically named script in the ./scripts/linux directory:
# replaces any reference to python with python3

import os

for script in os.listdir('./scripts'):
    if script.endswith('.sh'):
        with open(f'./scripts/{script}', 'r') as file:
            content = file.read()
            content = content.replace('python', 'python3')
            content = content.replace('python ', 'python3 ')
            content = content.replace('python\n', 'python3\n')
            content = content.replace('python3\n', 'python3\n')
            content = content.replace('python3 ', 'python3 ')
            content = content.replace('python3\n', 'python3\n')

            with open(f'./scripts/linux/{script}', 'w') as file:
                file.write(content)