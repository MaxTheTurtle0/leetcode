/**
 * @param {number} num
 * @return {number}
 */
function numberOfSteps(num) {
    let steps = 0;
    while (num > 0) {
        (num % 2 == 0) ? num /= 2 : num--;
        steps++;
    }
    return steps;
};

/**
 * @param {number} num
 * @return {number}
 */
function numberOfStepsBitwise(num) {
    let steps = 0;
    while (num > 0) {
        (num & 1) ? num >>= 1 : num--;
        steps++;
    }
    return steps;
};
