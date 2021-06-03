#Common imports
import pandas as pd
import numpy as np
import matplotlib.pyplot as plt

#making pandas dataframe
#df = pd.read_csv('../data/filename.csv')
#position = df["position"].to_list()

size = 1.0
position  = np.random.rand(1,10)
position = sorted(position)

step = position[1] - position[0]

max_pos = max(position)
norm = len(position)

    
print(position, max_pos)

norm = position[1:]**2/(2*np.pi*position[1:]*position[1] - np.pi*position[1]*position[1])


dr = 0.0

prob = np.zeros(size) 
counter = 0
p = 0

while p < norm:
    if(dr <= position[p] < dr + step):
        prob[counter] += 1
        p += 1
    else:
        counter += 1
        dr += step

norm = len(position)
prob = prob/(float(norm))










