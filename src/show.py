import matplotlib.pyplot as plt
import numpy as np
import csv

# Read data from CSV file into a numpy array
with open('./test.csv', 'r') as file:
    reader = csv.reader(file)
    data = np.array(list(reader), dtype=float)

# Plot the data
plt.imshow(data, cmap='gray', aspect='auto')
plt.colorbar()
plt.show()
