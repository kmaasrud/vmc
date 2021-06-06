#Common imports
import pandas as pd
import numpy as np
import matplotlib.pyplot as plt

#making pandas dataframe
df1 = pd.read_csv('../data/N2/onebody_with-jastro.csv')
df2 = pd.read_csv('../data/N2/onebody_without-jastro.csv')

rho_with = df1['wf_squared']
r_with = df1['distance']

rho_without = df2['wf_squared']
r_without = df2['distance']

#rho_with = sorted(rho_with)
#rho_without = sorted(rho_without)



norm_with = r_with[1:]**2/(2*np.pi*r_with[1:]*r_with[1] - np.pi*r_with[1]*r_with[1])
norm_without = r_without[1:]**2/(2*np.pi*r_without[1:]*r_without[1] - np.pi*r_without[1]*r_without[1])

with_jastrow    = rho_with[1:]*norm_with
without_jastrow = rho_without[1:]*norm_without

fig = plt.figure()

plt.plot(r_with[1:], with_jastrow/sum(with_jastrow), linestyle = 'None', marker = '.', label = 'With Jastrow factor')

plt.plot(r_without[1:], without_jastrow/sum(without_jastrow), linestyle = 'None', marker = '.', label = 'Without Jastrow factor')


plt.xlabel(r"$|\mathbf{r}|$", fontsize = 10)
plt.ylabel(r"$\rho(\mathbf{r})$", fontsize = 10)
plt.legend(fontsize = 10)

plt.savefig('../plots/onebodydensity_2p.png')
plt.show()



