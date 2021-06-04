import pandas as pd
import numpy as np


#return energy minima with corresponding alpha/beta from steepest decent
#Data: 
""" 
df = pd.read_csv('../data/filename.csv')
energy  = df['energy'].to_list()
alpha   = df['alpha'].to_list()
beta    = df['beta'].to_list()
alpha_woJ = df['alpha_woJ'].to_list() """

#Dummydata: 
energy = [1,4,5,6,6.5,7,-10]
alpha = [0.1, 0.2, 0.3, 0.5, 0, 1, -2]
alpha_woJ = [0.1, 0.2, 0.3, 0.5, 0, 1, -2]
beta = alpha 

#Finding energy min from list
energy_min          = min(energy)
energy_min_index    = energy.index(energy_min) #index of energy_min

#corresponding alpha/beta to energy min
alpha       = alpha[energy_min_index]
alpha_woJ   = alpha_woJ[energy_min_index]
beta        = beta[energy_min_index]

print(alpha)
print(beta)
print('                            energy_min [a.u]:  alpha:     alpha_woJ:   beta:   ')
print('Method_(non-)interacting:           {:.4f}        {:.3f}   {:.5f}   {:.6f}     '.format(float(energy_min), float(alpha), float(alpha_woJ), float(beta) ))



