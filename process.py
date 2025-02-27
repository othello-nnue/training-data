import tarfile
import re


# Regular expression pattern to match the specified line format
# 1. position with 36 empty squares
# 2. depth fed to Edax
# 3. strength fed to Edax (100 means complete analysis)
# 4. upper score bound
# 5. lower score bound
# 6. number of positions Edax explored.
# The upper and lower score bounds represent the true theoretical values when the depth is 36 and the strength is 100. If the strength is not 100, the same value is entered, indicating a predicted value. The number of positions Edax explored is positive when the command-line option given to Edax looks like
# A negative value means different command-line options were used, for instance, parallelized or different alpha and beta values.

pattern = re.compile(r'^[-OX]{64} X;(,-?\d+){5}$')

# obf,count,score,alpha,beta
def parse_line(line):

    if not pattern.match(line):
        raise ValueError("Unexpected format")

    # Split the line into character sequence and integer part
    char_sequence, integer_part = line.split(';')

    # Process the character sequence (if needed)
    # Here, we are just trimming white spaces
    char_sequence = char_sequence.strip()

    # Extract integers using regular expression
    integers = re.findall(r'-?\d+', integer_part)

    # Convert extracted strings to integers
    integers = [int(i) for i in integers]

    if integers[-1] < 0:
        raise ValueError("NOOOO")
    return char_sequence, integers

# Replace 'your_file.tar.bz2' with your actual file path
with tarfile.open('knowledge_archive.tar.bz2', 'r') as tar:
    for member in tar:
        f = tar.extractfile(member)
        if f is None:
            print(f"Debug: Skipping non-regular file: {member.name}")
            continue

        # Use 'with' statement when 'f' is not 'None'
        with f:
            # Iterate over each line in the file
            for line in f:  
                # Process each line here
                # For example, you could print the line
                x = line.decode('utf-8').strip()
                print(x)
                y = parse_line(x)
                # print(parse_line(x)) 
            #content = f.read()
            # Process the content here
            # For example, you could print the file name and its size
            #print(f"{member.name}: {len(content)} bytes")
