#Common imports
import pandas as pd
import numpy as np
import matplotlib.pyplot as plt

#making pandas dataframe
#df = pd.read_csv('../data/filename.csv')
#rho2 = df["rho2"].to_list()
#distance = df["distance"].to_list()
#max_r  = max(distance)
#radius = np.random.rand(1,max_r)


#dummydata
max_r = 10
radius = np.random.random_sample(size = 11)
rho2 = [0.01, 0.02, 0.03, 0.04, 0.05, 0.06, 0.07, 0.08, 0.09, 0.1]
rho2_1 = [0.05, 0.07, 0.09, 0.093, 0.096, 0.0966, 0.098, 0.099, 0.0999, 0.1]

rho2 = sorted(rho2)
rho2_1 = sorted(rho2_1)


norm = radius[1:]**2/(2*np.pi*radius[1:]*radius[1] - np.pi*radius[1]*radius[1])

with_jastrow    = rho2*norm
without_jastrow = rho2_1*norm

fig = plt.figure()

plt.plot(radius[1:], with_jastrow/sum(with_jastrow), linestyle = 'None', marker = 'o', label = 'With Jastrow factor')

plt.plot(radius[1:], without_jastrow/sum(without_jastrow), linestyle = 'None', marker = 'o', label = 'Without Jastrow factor')


plt.xlabel(r"$|\mathbf{r}|$", fontsize = 10)
plt.ylabel(r"$\rho(\mathbf{r})$", fontsize = 10)
plt.legend(fontsize = 10)

#plt.savefig('../plots/onebodydensity_2p.png')
plt.show()



