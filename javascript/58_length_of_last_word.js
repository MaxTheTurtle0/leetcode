/**
 * @param {string} s
 * @return {number}
 */
function lengthOfLastWord(s) {
    numOfWords = 0;
    for (let i = s.length - 1; i > -1; i--) {
        if (s[i] !== " ") numOfWords++; 
        else if (numOfWords > 0) return numOfWords;
    }
    return numOfWords;
};
