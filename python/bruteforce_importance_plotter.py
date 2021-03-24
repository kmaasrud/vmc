import pandas as pd
import plotter as plot


#step_size = [0.5, 0.25, 0.75, 0.125, 0.375, 0.626, 0.875, 1]
step_size = 1
dim = 1


DATA_DIR_BRUTEFORCE = rf"../data/bruteforce_vs_importance/BruteForceMetropolis/step_size{step_size}/{dim}D.csv"
DATA_DIR_IMPORTANCE = rf"../data/bruteforce_vs_importance/ImportanceMetropolis/step_size{step_size}/{dim}D.csv"
PLOT_DIR = fr"../plots/bruteforce_vs_importance/ImportanceMetropolis/step_size{step_size}/"
FILENEME_PLOT = f"{dim}D.png"


df_BRUTEFORCE = pd.read_csv(DATA_DIR_BRUTEFORCE)
df_IMPORTANCE = pd.read_csv(DATA_DIR_IMPORTANCE)

energy_BF = df_BRUTEFORCE["Energy"]
energy_IMP = df_IMPORTANCE["Energy"]

alpha_BF = df_BRUTEFORCE["Alpha"]
alpha_IMP = df_IMPORTANCE["Alpha"]


label = "energy vs. energy"
xlabel = "energy1"
ylabel = "energy2"
title = "some stupido plot"

plot.ploting(energy_BF, alpha_BF, label, xlabel, ylabel, title, PLOT_DIR, FILENEME_PLOT)


