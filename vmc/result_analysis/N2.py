import pandas as pd
import numpy as np


#Turning files into dataframes
df1 = pd.read_csv('../data/N2/BruteForceMetropolis_interacting_analytical.csv')
df2 = pd.read_csv('../data/N2/BruteForceMetropolis_interacting_numerical.csv')
df3 = pd.read_csv('../data/N2/BruteForceMetropolis_non-interacting_analytical.csv')
df4 = pd.read_csv('../data/N2/BruteForceMetropolis_non-interacting_numerical.csv')


#Avarage time over a set of calculations:
mean_t_1 = df1["time"].mean()
mean_t_2 = df2["time"].mean()
mean_t_3 = df3["time"].mean()
mean_t_4 = df4["time"].mean()


#statistical analysis
mean    = lambda x: sum(x) / len(x)
std     = lambda x: np.sqrt(sum(map(lambda y: (y - mean(x))**2, x)) / len(x))
se      = lambda x: std(x) / np.sqrt(len(x))
var     = lambda x: sum(map(lambda y: (y - mean(x))**2, x)) / len(x)


#readout energy from daraframe and convert to float
df1['energy'] = df1['energy'].astype(float)
df2['energy'] = df2['energy'].astype(float)
df3['energy'] = df3['energy'].astype(float)
df4['energy'] = df4['energy'].astype(float)

#Energy dataframe to list
energy1 = df1.energy.to_list()
energy2 = df2.energy.to_list()
energy3 = df3.energy.to_list()
energy4 = df4.energy.to_list()

#Calculate mean of energy
mean_e1 = mean(energy1)
mean_e2 = mean(energy2)
mean_e3 = mean(energy3)
mean_e4 = mean(energy4)

#Calculate variance of energy
variance1 = var(energy1)
variance2 = var(energy2)
variance3 = var(energy3)
variance4 = var(energy4)

print('                                 t_avg [s]:  Energy [a.u]:        Var:  ')
print('BFM_interacting_analytical:          {:.4f}          {:.2f}     {:.5f}  '.format(mean_t_1, mean_e1 ,variance1))
print('BFM_interacting_numerical:           {:.4f}          {:.2f}     {:.5f}  '.format(mean_t_2, mean_e2 ,variance2))
print('BFM_non-interacting_analytical:      {:.4f}          {:.2f}     {:.5f}  '.format(mean_t_3, mean_e3, variance3))
print('BFM_non-interacting_analytical:      {:.4f}          {:.2f}     {:.5f}  '.format(mean_t_4, mean_e4, variance4))



