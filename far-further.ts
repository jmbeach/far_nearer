import * as fs from 'fs';
import * as hsl from 'hsl-to-hex';

const farNearer = fs.readFileSync('far nearer_template.svg', 'utf-8');
const hue1 = 305
const color1SL = [100, 50]
const hue2 = 337
const color2SL = [100, 49]
const hue3 = 34
const color3SL = [100, 50]

const adjustColor = (color: number, amount: number) => {
  let result = color + amount;
  if (result > 360) { result = 360 - result }
  if (result < 0) { result = 360 + result }
  return result;
}

for (let i = 0; i < 360; i++) {
  const newColor1 = hsl(adjustColor(hue1, i), color1SL[0], color1SL[1] + 40);
  const newColor2 = hsl(adjustColor(hue2, i), color2SL[0], color2SL[1] + 40);
  const newColor3 = hsl(adjustColor(hue3, i), color3SL[0], color3SL[1] + 50);
  let result = farNearer.replace(/COLOR_1/g, newColor1)
  result = result.replace(/COLOR_2/g, newColor2)
  result = result.replace(/COLOR_3/g, newColor3)
  fs.writeFileSync(`generated/far_nearer${i}.svg`, result)
}