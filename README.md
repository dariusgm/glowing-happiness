# glowing-happiness
Analyse the distribution of files and its content. Note that the order is not stable.

## building
### install rust
```bash
sudo apt update -y && sudo apt upgrade -y
sudo apt install curl build-essential gcc make -y
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### build project
```bash
cargo build
```


## usage
When running as binary directly, replace `cargo run -- ` with `glowing-happiness `  

### Input
Provide the input path of the directory / repository that you want to analyse.
This command will return a json with the tool distribution.
```bash
# when using the ready binary
cargo run -- --input .
```
Output: 

```json
{"c":3,"json":261,"spring-boot":1,"test":14,"rust":7,"python":2,"objective-c":1,"gitignore":2,"javascript":261,"circleci":1,"git":1,"xml":3,"toml":2,"swift":1,"cargo":2,"github":1,"markdown":1,"spark":1}
```

For Visualisation, see [Visualisation](README.md#Visualisation)

### Mode
With `mode` you can define the aggregation level of the output.
When setting no mode is defaults to "count_by_tool".
Both calls are equivalent:

#### No Mode / count_by_tool
```bash
cargo run -- --input .
```

```bash
cargo run -- --input . --mode count_by_tool
```

### list_by_file
```bash
cargo run -- --input . --mode list_by_file
```


#### List
Returns only a distinct json list of tools used:

```bash
cargo run -- --input . --mode list
```

```json
["toml","gitignore","spark","json","test","github","c","cargo","javascript","rust","python","swift","git","spring-boot","markdown","objective-c","xml","circleci"]

```

# Snippet
To execute the application for several repositories, you can glue it with some python:
```python
import os
import subprocess
root = "/home/darius"
for directory in os.listdir(root):
    subprocess.call(f"cargo run -- --input {os.path.join(root, directory)} > {directory}.json", shell=True)
```
you can also run it in bash with awk magic
```bash
ls -dl /home/darius/*/ | awk -F'[[:space:]]' '{print "cargo run -- --input " $NF " > " substr($NF, 1, length($NF)-1) ".json"}' | bash
```

Now you can visualize it. But before we go to it, some more examples of real repositories.

# Real World example
Here you will find some popular repositories and the execution times of the analysis.
Please note as with every benchmark, this can only give you a idea how fast it will run on your device.
Also note that this repositories are very huge. The fetching of the content will take longer than the analysis.

## VS Code
```bash
git clone --branch main https://github.com/Microsoft/vscode/ repositories/vscode
find repositories/vscode | wc -l
time cargo run -- --input repositories/vscode
```

Example Output:
```json
{"objective-c":1,"java":1,"npm":107,"xml property list":3,"docker":2,"dart":1,"yaml":58,"rust":2,"python":2,"shell":45,"gitignore":18,"javascript":248,"svg":72,"css":211,"github":2,"json":644,"swift":1,"xml":5,"git":1,"jupyter notebook":1,"yarn":100,"markdown":75,"go":2,"png":71,"html":43,"typescript":3987,"c":1}
```

4 seconds for 8k files


## Flutter (300 MB locally)
```bash
git clone --branch master https://github.com/flutter/flutter.git repositories/flutter
find repositories/flutter | wc -l
time cargo run -- --input repositories/flutter
```

4 seconds for 8800 files.

# Visualisation
You can use the streamlit code in python to visualize the repositories.
Something like this may get you on the way.
```bash
cargo run -- --input . > app_1.json
python3 -m venv ./venv
. ./venv/bin/activate
pip3 install -r requirements.txt
streamlit run streamlit/app.py
```

![streamlit](streamlit.png)

In case you want a rust solution here, feel free to contribute. 

# Parallel Execution
The files will be analysed in parallel. To reduce the IO load on your device, you can set
`RAYON_NUM_THREADS=4` to only use 4 threads instead of all.
