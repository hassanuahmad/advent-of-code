const fs = require("fs");

fs.readFile("input.txt", (err, data) => {
    if (err) throw err;

    const inputData = data.toString().replace(/\r\n/g, "\n").split("\n");
    const numInputData = inputData.map((i) => Number(i));

    let counter = 0;

    for (let i = 0; i < numInputData.length; i++) {
        if (numInputData[i + 1] > numInputData[i]) counter++;
    }
    console.log(counter);
});
