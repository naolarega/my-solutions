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