const hexString =
  "ea14b1cc450be7cef2859d899843f8998d22ef1e586aaebf1d506b44d7104e9a";

function hexStringToByteArray(hex: string): number[] {
  let bytes = [];
  for (let i = 0; i < hex.length; i += 2) {
    bytes.push(parseInt(hex.substring(i, i + 2), 16));
  }
  return bytes;
}

const byteArray = hexStringToByteArray(hexString);
console.log(byteArray);
