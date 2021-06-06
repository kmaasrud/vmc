import pandas as pd
import numpy as np

from lib.blocking import block

df1 = pd.read_csv('../data/N2/omega1_alpha0.98_beta0.43_without-jastrow.csv')
df2 = pd.read_csv('../data/N2/omega0.5_alpha0.97_beta0.38_without-jastrow.csv')
df3 = pd.read_csv('../data/N2/omega0.1_alpha0.97_beta0.35_without-jastrow.csv')
df4 = pd.read_csv('../data/N2/omega0.05_alpha0.98_beta0.24_without-jastrow.csv')
df5 = pd.read_csv('../data/N2/omega0.01_alpha0.93_beta0.16_without-jastrow.csv')


energy1 = df1['energy[au]']
energy2 = df2['energy[au]']
energy3 = df3['energy[au]']
energy4 = df4['energy[au]']
energy5 = df5['energy[au]']


kinetic1 = df1['kinetic']
kinetic2 = df2['kinetic']
kinetic3 = df3['kinetic']
kinetic4 = df4['kinetic']
kinetic5 = df5['kinetic']

variance1 = df1['variance']
variance2 = df2['variance']
variance3 = df3['variance']
variance4 = df4['variance']
variance5 = df5['variance']

acceptance_ratio1 = df1['acceptance_rate']
acceptance_ratio2 = df2['acceptance_rate']
acceptance_ratio3 = df3['acceptance_rate']
acceptance_ratio4 = df4['acceptance_rate']
acceptance_ratio5 = df5['acceptance_rate']

distance1 = df1['avg_distance']
distance2 = df2['avg_distance']
distance3 = df3['avg_distance']
distance4 = df4['avg_distance']
distance5 = df5['avg_distance']


print(type(distance5))


print('omega  Energy[a.u]:   kinetic:  variance:  accept:    dist: ')
print('1.0:     {:f}   {:f}   {:f}   {:f}   {:f} '.format(float(energy1), float(kinetic1), float(variance1), float(acceptance_ratio1), float(distance1)))
print('0.5:     {:f}   {:f}   {:f}   {:f}   {:f} '.format(float(energy2), float(kinetic2), float(variance2), float(acceptance_ratio2), float(distance2)))
print('0.3:     {:f}   {:f}   {:f}   {:f}   {:f} '.format(float(energy3), float(kinetic3), float(variance3), float(acceptance_ratio3), float(distance3)))
print('0.05:    {:f}   {:f}   {:f}   {:f}   {:f} '.format(float(energy4), float(kinetic4), float(variance4), float(acceptance_ratio4), float(distance4)))
print('0.05:    {:f}   {:f}   {:f}   {:f}   {:f} '.format(float(energy5), float(kinetic5), float(variance5), float(acceptance_ratio5), float(distance5)))






