import pandas as pd
import os
import matplotlib.pyplot as plt


import numpy as np


#location for files and plots

PLOT_DIR = "../plots/"
DATA_DIR = "../data"
FILENAME_PLOT = 'SGD_alphas'
PLOT_DIR = "./"

class color():
    colors = ["tab:blue", "tab:orange", "tab:green", "tab:red", "tab:purple", "tab:brown", "tab:pink", "tab:gray", "tab:olive", "tab:cyan"]

    def give(self):
        c = self.colors[0]
        self.colors = self.colors[1:]
        return c

#figure size and resolution
fig = plt.figure()
#plt.style.use("seaborn")
#colour, linewith, linestyle
#boundaries
#plt.xlim(min(x)*1.1, max(x)*1.1)
#plt.ylim(0.15, 0.95)
#legend
plt.legend(loc = 'best', prop = {'size':14}, frameon = False)
plt.rc('font', size=10)
plt.rc('axes', titlesize=12)
plt.xlabel("Iterations")
#plt.ylabel(r"$\alpha$")

color = color()

omegas = ["1", "0.5", "0.1", "0.05", "0.01"]


Energies = []
filenames = []
for o in omegas:
    filename = f"o-{o}.csv"
    print(filename)
    DATA_DIR = "./data/sgd/omega/"+filename

    df = pd.read_csv(DATA_DIR)

    alpha = df["alpha"]
    beta = df["beta"]
    energy = df["energy-per-particle[au]"]
    x = range(len(beta))
    c = color.give()
    plt.plot(x, beta, label  = r"$\omega$: %.2f, $\mathbf{\beta}$" %float(o), linewidth = 2, c = c)
    plt.plot(x, alpha, label =  r"$\omega$: %.2f, $\mathbf{\alpha}$" %float(o), linewidth = 2, c = c, alpha = 0.6)
    plt.plot(x, energy, label =  r"$\omega$: %.2f, $\mathbf{Energy}$" %float(o), linewidth = 2, c = c, alpha = 0.3)


#df = pd.read_csv("data/sgd/start_params/" + mine_name)
#plt.plot(range(len(alpha)), alpha, label = r"$\alpha$")
#plt.plot(range(len(beta)), beta, label = r"$\beta$")
#plt.plot(range(len(energy)), energy, label = "Energy")
plt.title(r"SGD: $\alpha$ and $\beta$ for different $\omega$")

plt.legend(bbox_to_anchor=(1.05, 1), loc='upper left')
plt.draw()
plt.show()
#plt.save_fig(PLOT_DIR + "SGD_start-alpha.png")
