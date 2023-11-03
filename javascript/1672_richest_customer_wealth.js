/**
 * @param {number[][]} accounts
 * @return {number}
 */
function maximumWealth(accounts) {
    let maximumWealth = 0; 
    for (let i = 0; i < accounts.length; i++) {
        let customerWealth = 0;
        accounts[i].forEach(num => customerWealth +=num);
        if (maximumWealth < customerWealth) maximumWealth = customerWealth;
    }
    return maximumWealth;
};
