# Spring Mass Damper FMU

This is an example of a Spring Mass Damper Functional Mock-Up Unit (FMU) that solves the spring mass damper state space and then integrates using Euler integration.

## Getting Started


### The Maths

The equation for a generic spring mass damper system is as follows:

$$\begin{equation}m\ddot{x} + c\dot{x} + kx = F \end{equation}$$

It is a 2nd Order Differential equation. The state space model requires us to transform the equation into a combination of 1st Order Differential Equations. We can do this by defining:

$$
\begin{align}
  x_1 &= x \\\\
  x_2 &= \dot{x}
\end{align}
$$

And finding this derivatives of these:

$$
\begin{align}
  \dot{x}_1 &= \dot{x} = x_2  \\\\
  \dot{x}_2 &= \ddot{x}
\end{align}
$$

We can the substitute and re-arrange to get $ \dot{x} $ in the form of its states $x_{1,2}$.

$$
\begin{align}
  \dot{x}_1 &= x_2 \\\\
  \dot{x}_2 &= -\frac{k}{m}x_1 - \frac{c}{m}x_2 + \frac{1}{m}F
\end{align}
$$

We now have the equation in the form $\dot{x} = Ax + Bu$:

$$
\begin{equation}
\begin{bmatrix}
\dot{x}_1 \\\\ \dot{x}_2
\end{bmatrix} =
\begin{bmatrix}
0 & 1 \\\\
\frac{-k}{m} & \frac{1}{m}
\end{bmatrix} 
\begin{bmatrix}
x_1 \\\\ x_2
\end{bmatrix} +
\begin{bmatrix}
0 & \frac{1}{m} \\\\
\end{bmatrix}
\begin{bmatrix} F \end{bmatrix}
\end{equation}
$$

We can compute this at each timestep and integrate using Euler integration to get our new state:

$$
\begin{equation}
\begin{bmatrix}
x_{1} \\\\ x_{2}
\end{bmatrix} =
\begin{bmatrix}
\dot{x}_1 \\\\ \dot{x}_2
\end{bmatrix}
\begin{bmatrix} dt \end{bmatrix}
\end{equation}
$$
