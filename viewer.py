import matplotlib.pyplot as plt
import csv

with open('output.txt', 'r') as csvfile:
    reader = csv.reader(csvfile, delimiter=';')
    for row in reader:
        xs = []
        ys = []

        print(row[0])
        for pixel in row[1:-1]:
            x, y = pixel.strip().split(" ")
            xs.append(int(x))
            ys.append(int(y))

        plt.plot(xs[0], ys[0], 'x')
        plt.plot(xs, ys, '-')

plt.show()
