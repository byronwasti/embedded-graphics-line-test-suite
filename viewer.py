import matplotlib.pyplot as plt
import seaborn as sns
import csv

with open('output.txt', 'r') as csvfile:
    reader = csv.reader(csvfile, delimiter=';')
    for row in reader:
        xs = []
        ys = []

        for pixel in row[1:-1]:
            x, y = pixel.strip().split(" ")
            xs.append(int(x))
            ys.append(int(y))

        xs.sort()
        ys.sort()
        plt.plot(xs, ys, '-')

plt.show()
