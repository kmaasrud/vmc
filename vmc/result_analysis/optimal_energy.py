import pandas as pd
import numpy as np

from lib.blocking import block

df1 = pd.read_csv('../data/N2/omega1_alpha0.98_beta0.43.csv')
df2 = pd.read_csv('../data/N2/omega0.5_alpha0.97_beta0.38.csv')
df3 = pd.read_csv('../data/N2/omega0.1_alpha0.97_beta0.35.csv')
df4 = pd.read_csv('../data/N2/omega0.05_alpha0.98_beta0.24.csv')
df5 = pd.read_csv('../data/N2/omega0.01_alpha0.93_beta0.16.csv')


energy1 = df1['energy[au]'].mean()
energy2 = df2['energy[au]'].mean()
energy3 = df3['energy[au]'].mean()
energy4 = df4['energy[au]'].mean()
energy5 = df5['energy[au]'].mean()



kinetic1 = df1['kinetic'].mean()
kinetic2 = df2['kinetic'].mean()
kinetic3 = df3['kinetic'].mean()
kinetic4 = df4['kinetic'].mean()
kinetic5 = df5['kinetic'].mean()


variance1 = df1['variance'].mean()
variance2 = df2['variance'].mean()
variance3 = df3['variance'].mean()
variance4 = df4['variance'].mean()
variance5 = df5['variance'].mean()


acceptance_ratio1 = df1['acceptance_rate'].mean()
acceptance_ratio2 = df2['acceptance_rate'].mean()
acceptance_ratio3 = df3['acceptance_rate'].mean()
acceptance_ratio4 = df4['acceptance_rate'].mean()
acceptance_ratio5 = df5['acceptance_rate'].mean()

mean    = lambda x: sum(x) / len(x)
std     = lambda x: np.sqrt(sum(map(lambda y: (y - mean(x))**2, x)) / len(x))
se      = lambda x: std(x) / np.sqrt(len(x))
var     = lambda x: sum(map(lambda y: (y - mean(x))**2, x)) / len(x)


(mean1,var1) = block(df1['energy[au]'])
(mean2,var2) = block(df2['energy[au]'])
(mean3,var3) = block(df3['energy[au]'])
(mean4,var4) = block(df4['energy[au]'])
(mean5,var5) = block(df5['energy[au]'])


print(type(var1))


print('omega  Energy[a.u]:   kinetic:  variance:  accept: ')
print('1.0:     {:f}   {:f}   {:f}   {:f}   '.format(energy1, kinetic1, variance1, acceptance_ratio1))
print('0.5:     {:f}   {:f}   {:f}   {:f}   '.format(energy2, kinetic2, variance2, acceptance_ratio2))
print('0.1:     {:f}   {:f}   {:f}   {:f}   '.format(energy3, kinetic3, variance3, acceptance_ratio3))
print('0.05:    {:f}   {:f}   {:f}   {:f}   '.format(energy4, kinetic4, variance4, acceptance_ratio4))
print('0.01:    {:f}   {:f}   {:f}   {:f}   '.format(energy5, kinetic5, variance5, acceptance_ratio5))






