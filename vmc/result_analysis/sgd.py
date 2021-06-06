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




start_betas = ["0.4", "0.6", "0.8", "1", "1.2", "1.4", "1.6", "1.8"]
start_alphas= ["0.4", "0.6", "0.8", "1", "1.2", "1.4", "1.6", "1.8"]


Energies = []
filenames = []
for start_beta in start_betas:
    for start_alpha in start_alphas[0:6]:
        filename = f"a-{start_alpha}_b-{start_beta}.csv"
        print(filename)
        DATA_DIR = "./data/sgd/start_params/"+filename

        df = pd.read_csv(DATA_DIR)

        alpha = df["alpha"]
        beta = df["beta"]
        energy = df["energy-per-particle[au]"]
        Energies.append(energy.iloc[-1])
        filenames.append(filename)

mine_name = []

mine = min(Energies)
indx = Energies.index(mine)
Energies[indx] = 100
mine_name.append(filenames[indx])
print(Energies)
mine = min(Energies)
indx = Energies.index(mine)
Energies[indx] = 100
mine_name.append(filenames[indx])

start_alpha = "0.4"

for name in mine_name:

    DATA_DIR = f"./data/sgd/start_params/" + name

    df = pd.read_csv(DATA_DIR)

    alpha = df["alpha"]
    beta = df["beta"]
    energy = df["energy-per-particle[au]"]
    x = range(len(alpha))
    c = color.give()
    print(name[2:5])
    a = float(name[2:5])
    b = float(name[8:11])
    print(name)
    plt.plot(x, beta, label  = r"$\alpha_i$: %.1f, $\beta_i$: %.1f, $\mathbf{\beta}$" %(a,b), linewidth = 2, c = c)
    plt.plot(x, alpha, label =  r"$\alpha_i$: %.1f, $\beta_i$: %.1f, $\mathbf{\alpha}$" %(a,b), linewidth = 2, c = c, alpha = 0.6)
    plt.plot(x, energy, label =  r"$\alpha_i$: %.1f, $\beta_i$: %.1f, $\mathbf{Energy}$" %(a,b), linewidth = 2, c = c, alpha = 0.3)


#df = pd.read_csv("data/sgd/start_params/" + mine_name)
#plt.plot(range(len(alpha)), alpha, label = r"$\alpha$")
#plt.plot(range(len(beta)), beta, label = r"$\beta$")
#plt.plot(range(len(energy)), energy, label = "Energy")
plt.title(r"SGD: Two lowest Energy simulations")

plt.legend()
plt.draw()
plt.show()
#plt.save_fig(PLOT_DIR + "SGD_start-alpha.png")
