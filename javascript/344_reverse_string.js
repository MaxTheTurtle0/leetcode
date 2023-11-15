/**
* @param {character[]} s
* @return {void} Do not return anything, modify s in-place instead.
*/
function reverseString(s) {
    for (let i = 0; i < Math.floor(s.length / 2); i++) {
        let tmp = s[i];
        s[i] = s[s.length - 1 - i];
        s[s.length - 1 - i] = tmp;
    }
};
