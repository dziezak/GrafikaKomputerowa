# Raport: Działanie sceny OpenGL z oświetleniem, kamerą i lustrem

## 1. Cel programu
Program implementuje trójwymiarową scenę (układ planetarny wraz ze statkiem kosmicznym) w OpenGL 3.3, z wykorzystaniem następujących mechanizmów:
* Wielu źródeł światła (słońce, reflektory, światło górne (opcjonalnie w przyszłości na razie nie)).
* Modeli oświetlenia Phong oraz Blinn–Phong.
* Efektu mgły (fog).
* Kilku trybów kamery.
* Powierzchni lustra zrealizowanej przy użyciu **stencil buffer** oraz macierzy odbicia.

---

## 2. Układy współrzędnych (przestrzenie)
Przepływ wierzchołka przez kolejne przestrzenie w programie:
**Model Space** $\rightarrow$ **World Space** $\rightarrow$ **View Space** $\rightarrow$ **Clip Space** $\rightarrow$ **NDC** $\rightarrow$ **Screen Space**.

---

## 3. Macierze transformacji

### 3.1 Macierz modelu ($M$)
Opisuje położenie, skalę oraz obrót obiektu. Ogólna postać:
$$M = T \cdot R \cdot S$$

Dla planety na orbicie współrzędne translacji $T$ obliczane są dynamicznie:
$$x = \cos(\text{angle}) \cdot \text{radius}$$
$$z = \sin(\text{angle}) \cdot \text{radius}$$

$$T = \begin{bmatrix} 1 & 0 & 0 & x \\ 0 & 1 & 0 & y \\ 0 & 0 & 1 & z \\ 0 & 0 & 0 & 1 \end{bmatrix}$$

### 3.2 Macierz widoku ($V$)
Przenosi scenę do przestrzeni kamery (View Space). Kamera w tym układzie zawsze znajduje się w punkcie $(0, 0, 0)$. Pozycja fragmentu w przestrzeni kamery:
$$\text{FragPos}_{cam} = V \cdot M \cdot \begin{bmatrix} x \\ y \\ z \\ 1 \end{bmatrix}$$



### 3.3 Macierz projekcji ($P$)
Stosowana jest projekcja perspektywiczna, definiująca ostrosłup widzenia (frustum):
$$P = \begin{bmatrix} \frac{1}{a \cdot \tan(\theta/2)} & 0 & 0 & 0 \\ 0 & \frac{1}{\tan(\theta/2)} & 0 & 0 \\ 0 & 0 & -\frac{f+n}{f-n} & -\frac{2fn}{f-n} \\ 0 & 0 & -1 & 0 \end{bmatrix}$$

---

## 4. Macierz normalnych ($N$)
Aby wektory normalne zachowywały poprawny kierunek po transformacjach (szczególnie skalowaniu), stosujemy macierz normalnych:
$$N = (V \cdot M)^{-1 \top}$$
W shaderze: `mat3 normalMatrix = mat3(transpose(inverse(view * model)));`

---

## 5. Modele oświetlenia

### 5.1 Model Phonga i Blinn-Phong
Oba modele obliczają światło spekularne (odbłysk). **Blinn-Phong** wykorzystuje wektor połówkowy ($h$):
$$\mathbf{h} = \frac{\mathbf{l} + \mathbf{v}}{\|\mathbf{l} + \mathbf{v}\|}$$
Komponent spekularny: $I_s = (\mathbf{n} \cdot \mathbf{h})^{\text{shininess} \cdot 4}$.



### 5.2 Reflektory (Spotlight)
Intensywność światła w obrębie stożka zależy od kąta $\theta$ i wygładzania krawędzi:
$$I = \text{clamp}\left( \frac{\theta - \text{outerCutOff}}{\text{cutOff} - \text{outerCutOff}}, 0, 1 \right)$$

---

## 6. Efekt Mgły (Fog)
Mgła zależy od odległości od kamery (głębi):
$$\text{fogFactor} = e^{-\text{depth} \cdot \text{density}}$$
Ostateczny kolor: `color = mix(fogColor, objectColor, fogFactor)`.

---

## 7. System Lustra

### 7.1 Stencil Buffer (Maska)
1. **Zapis**: Rysujemy lustro do bufora stencil, oznaczając piksele wartością $1$.
2. **Użycie**: Rysujemy odbitą scenę tylko tam, gdzie stencil wynosi $1$.

### 7.2 Macierz odbicia lustra
Odbicie realizowane jest przez transformację macierzy widoku względem płaszczyzny lustra ($p$):
$$V' = V \cdot T(p) \cdot S(1, 1, -1) \cdot T(-p)$$

Gdzie macierz skali $S$ odpowiada za negację osi Z:
$$S = \begin{bmatrix} 1 & 0 & 0 & 0 \\ 0 & 1 & 0 & 0 \\ 0 & 0 & -1 & 0 \\ 0 & 0 & 0 & 1 \end{bmatrix}$$

---

## 8. Kolejność renderowania (Pipeline)
1. Czyszczenie buforów: `COLOR`, `DEPTH`, `STENCIL`.
2. Renderowanie obiektów głównych (planety, słońce, statek).
3. Renderowanie mgły globalnej.
4. **Maska lustra**: Rysowanie lustra do Stencil Buffera.
5. **Odbicie**: Renderowanie sceny z macierzą $V'$ i odwróconym cullingiem (`glCullFace(GL_FRONT)`).
6. **Powierzchnia**: Renderowanie półprzezroczystej tafli lustra z blendingiem.