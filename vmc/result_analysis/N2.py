import pandas as pd

df1 = pd.read_csv('../data/N2/BruteForceMetropolis_interacting_analytical.csv')
df2 = pd.read_csv('../data/N2/BruteForceMetropolis_interacting_numerical.csv')
df3 = pd.read_csv('../data/N2/BruteForceMetropolis_non-interacting_analytical.csv')
df4 = pd.read_csv('../data/N2/BruteForceMetropolis_non-interacting_numerical.csv')


#Avarage time:
mean_t_1 = df1["time"].mean()
mean_t_2 = df2["time"].mean()
mean_t_3 = df3["time"].mean()
mean_t_4 = df4["time"].mean()

print('Avarage time: ')
print('BFM_interacting_analytical: {mean_t_1}')
print('BFM_interacting_numerical: {mean_t_2}')
print('BFM_non-interacting_analytical: {mean_t_3}')
print('BFM_non-interacting_analytical: {mean_t_4}')


#variance
energy1 = df1['energy']
var = lambda x: sum(map(lambda y: (y - mean(x))**2, x) / len(x))

var_energy = var(energy1)
print(var_energy)
