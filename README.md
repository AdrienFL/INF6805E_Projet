In order to run the code :

1. Compile the desired .bzz file using bzzc
2. In `projet.argos`, replace the values bytecode_file and debug_file with the corresponding files from previous step
3. Run `run_oneshot_nostdout.sh` to generate `experiment_data.csv` and `experiment_map.csv`
4. Install (preferably in a venv) the dependencies of `requirements.txt`. We tested using Python 3.14.2
5. Run the cells in order in the `analysis.ipynb` notebook to get the insights