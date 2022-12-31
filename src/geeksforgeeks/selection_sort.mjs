import test from 'node:test';
import assert from 'node:assert';

class Solution {
    /**
     * 
     * @param {[number]} arr 
     * @param {number} i 
     */
    select(arr,i){
        for(let j = i; j < arr.length; j++) {
            if(arr[j] < arr[i]) {
                const temp = arr[j];
                arr[j] = arr[i];
                arr[i] = temp;
            }
        }
    }

    /**
     * 
     * @param {[number]} arr 
     * @param {number} n 
     */
    selectionSort(arr,n){
        for(let i = 0; i < n; i++) {
            this.select(arr, i);
        }
    }
}

test('Test selction sort', () => {
    const selectionSortSolution = new Solution();

    const unsortedList = [4, 1, 3, 9, 7];

    selectionSortSolution.selectionSort(unsortedList, 5);

    assert.deepEqual(unsortedList, [1, 3, 4, 7, 9]);
});