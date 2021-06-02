import pandas as pd
import numpy as np

from lib.blocking import block

#Turning files into dataframes
#Brute Force
df1 = pd.read_csv('../data/N2/BruteForceMetropolis_interacting_analytical.csv')
df2 = pd.read_csv('../data/N2/BruteForceMetropolis_interacting_numerical.csv')
df3 = pd.read_csv('../data/N2/BruteForceMetropolis_non-interacting_analytical.csv')
df4 = pd.read_csv('../data/N2/BruteForceMetropolis_non-interacting_numerical.csv')

#Importance sampling
df5 = pd.read_csv('../data/N2/ImportanceMetropolis_interacting_analytical.csv')
df6 = pd.read_csv('../data/N2/ImportanceMetropolis_interacting_numerical.csv')
df7 = pd.read_csv('../data/N2/ImportanceMetropolis_non-interacting_analytical.csv')
df8 = pd.read_csv('../data/N2/ImportanceMetropolis_non-interacting_numerical.csv')


#Avarage time over a set of calculations:
mean_t_1 = df1["time[s]"].mean()
mean_t_2 = df2["time[s]"].mean()
mean_t_3 = df3["time[s]"].mean()
mean_t_4 = df4["time[s]"].mean()

mean_t_5 = df5["time[s]"].mean()
mean_t_6 = df6["time[s]"].mean()
mean_t_7 = df7["time[s]"].mean()
mean_t_8 = df8["time[s]"].mean()


#statistical analysis
mean    = lambda x: sum(x) / len(x)
std     = lambda x: np.sqrt(sum(map(lambda y: (y - mean(x))**2, x)) / len(x))
se      = lambda x: std(x) / np.sqrt(len(x))
var     = lambda x: sum(map(lambda y: (y - mean(x))**2, x)) / len(x)


#readout energy from daraframe and convert to float
""" df1['energy[au]'] = df1['energy[au]'].astype(float)
df2['energy[au]'] = df2['energy[au]'].astype(float)
df3['energy[au]'] = df3['energy[au]'].astype(float)
df4['energy[au]'] = df4['energy[au]'].astype(float)

df5['energy[au]'] = df5['energy[au]'].astype(float)
df6['energy[au]'] = df6['energy[au]'].astype(float)
df7['energy[au]'] = df7['energy[au]'].astype(float)
df8['energy[au]'] = df8['energy[au]'].astype(float) """

#Energy dataframe to list
energy1 = df1['energy[au]'].to_list()
energy2 = df2['energy[au]'].astype(float).to_list()
energy3 = df3['energy[au]'].astype(float).to_list()
energy4 = df4['energy[au]'].astype(float).to_list()

energy5 = df5['energy[au]'].astype(float).to_list()
energy6 = df6['energy[au]'].astype(float).to_list()
energy7 = df7['energy[au]'].astype(float).to_list()
energy8 = df8['energy[au]'].astype(float).to_list()

#Calculate mean of energy
mean_e1 = mean(energy1)
mean_e2 = mean(energy2)
mean_e3 = mean(energy3)
mean_e4 = mean(energy4)

mean_e5 = mean(energy5)
mean_e6 = mean(energy6)
mean_e7 = mean(energy7)
mean_e8 = mean(energy8)

#Calculate variance of energy
variance1 = var(energy1)
variance2 = var(energy2)
variance3 = var(energy3)
variance4 = var(energy4)

variance5 = var(energy5)
variance6 = var(energy6)
variance7 = var(energy7)
variance8 = var(energy8)

#blocking

(mean1,var1) = block(energy1)
(mean2,var2) = block(energy2)
(mean3,var3) = block(energy3)
(mean4,var4) = block(energy4)

(mean5,var5) = block(energy5)
(mean6,var6) = block(energy6)
(mean7,var7) = block(energy7)
(mean8,var8) = block(energy8)

std1 = np.sqrt(var1)
std2 = np.sqrt(var2)
std3 = np.sqrt(var3)
std4 = np.sqrt(var4)

std5 = np.sqrt(var5)
std6 = np.sqrt(var6)
std7 = np.sqrt(var7)
std8 = np.sqrt(var8)




print('                                 t_avg [s]:  Energy[a.u]:      Var:  block: ')
print('BFM_interacting_analytical:          {:.4f}        {:.2f}   {:.5f}   {:.2f}'.format(mean_t_1, mean_e1 ,variance1, var1))
print('BFM_interacting_numerical:           {:.4f}        {:.2f}   {:.5f}   {:.2f}'.format(mean_t_2, mean_e2 ,variance2, var2))
print('BFM_non-interacting_analytical:      {:.4f}        {:.2f}   {:.5f}   {:.2f}'.format(mean_t_3, mean_e3, variance3, var3))
print('BFM_non-interacting_analytical:      {:.4f}        {:.2f}   {:.5f}   {:.2f}'.format(mean_t_4, mean_e4, variance4, var4))
print('')
print('IS_interacting_analytical:           {:.4f}        {:.2f}   {:.5f}   {:.2f}'.format(mean_t_5, mean_e5 ,variance5, var5))
print('IS_interacting_numerical:            {:.4f}        {:.2f}   {:.5f}   {:.2f}'.format(mean_t_6, mean_e6 ,variance6, var6))
print('IS_non-interacting_analytical:       {:.4f}        {:.2f}   {:.5f}   {:.2f}'.format(mean_t_7, mean_e7, variance7, var7))
print('IS_non-interacting_analytical:       {:.4f}        {:.2f}   {:.5f}   {:.2f}'.format(mean_t_8, mean_e8, variance8, var8))


