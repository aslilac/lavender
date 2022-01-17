# VRAM, Palette, and Object details

Even though we just covered the IO registers, we have a few more to cover that
are related to graphics

## REG_DISPCNT: Display Control

<table>
  <thead>
    <tr>
      <th>f</th>
      <th>e</th>
      <th>d</th>
      <th>c</th>
      <th>b</th>
      <th>a</th>
      <th>9</th>
      <th>8</th>
      <th>7</th>
      <th>6</th>
      <th>5</th>
      <th>4</th>
      <th>3</th>
      <th>2</th>
      <th>1</th>
      <th>0</th>
    </tr>
  <tbody>
    <tr>
      <td>Object window</td>
      <td>Window 1</td>
      <td>Window 0</td>
      <td>Object layer</td>
      <td>Background layer 3</td>
      <td>Background layer 2</td>
      <td>Background layer 1</td>
      <td>Background layer 0</td>
      <td>Force screen blank</td>
      <td>Object mapping</td>
      <td>HB OAM unlock</td>
      <td>Page select</td>
      <td>Game Boy mode</td>
      <td colspan=3>Mode</td>
    </tr>
  </tbody>
</table>

## Graphics mode

-   Mode 3: 15-bit bitmap
-   Mode 4: Palette bitmap
