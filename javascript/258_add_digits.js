/**
 * @param {number} num
 * @return {number}
 */
function addDigits(num) {
    num = num.toString().split("").reduce((a, b) => parseInt(a) + parseInt(b));
    if (num > 9) return addDigits(num);
    return num;
};


/**
 * @param {number} num
 * @return {number}
 */
function addDigitsSmart(num) {
    while (num > 9) {
        num = num % 10 + Math.floor(num / 10);
    }
    return num;
};
