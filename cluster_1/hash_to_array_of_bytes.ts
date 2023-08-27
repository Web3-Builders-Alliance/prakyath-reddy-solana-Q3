const hexString =
  "74ef874a9fa69a86e091ea6dc2668047d7e102d518bebed19f8a3958f664e3da";

function hexStringToByteArray(hex: string): number[] {
  let bytes = [];
  for (let i = 0; i < hex.length; i += 2) {
    bytes.push(parseInt(hex.substring(i, i + 2), 16));
  }
  return bytes;
}

const byteArray = hexStringToByteArray(hexString);
console.log(byteArray);
