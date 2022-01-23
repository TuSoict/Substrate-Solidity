pub fn insertion_sort(arr: &mut [f64],size:usize) -> &mut[f64] {
    for i in 1..size{
        let mut j = i;
        while j>0 && arr[j] < arr[j-1]{
            arr.swap(j-1,j);
            j = j -1;
            
        }
    }
    arr
}

pub fn selection_sort(arr: &mut [f64],size:usize) -> &mut[f64] {
    for i in 0..size-1{
        let mut min:usize =i;
        for j in i+1..size{
            if arr[j]<arr[min] { min = j;}
        } 
        arr.swap(i,min);
    }

    arr
}


pub fn bubble_sort(arr: &mut [f64],size:usize) -> &mut[f64] {
    let mut sorted = false;
    for i in 1..size{
        if !sorted {
            sorted = true;
            for  j in 0..size-i{
                if arr[j]>arr[j+1]{
                    arr.swap(j,j+1);
                    sorted = false;
                }
            
            } ;
        }
    }

    arr
}

pub fn merge_sort(arr: &mut [f64],first:usize, last:usize) -> &mut [f64] {
    if first<last{
        let mid = (first+last)/2;
        merge_sort(arr,first,mid);
        merge_sort(arr,mid+1,last);
        merge(arr,first,mid,last);
    }

    arr
}
fn merge(arr:&mut [f64],first:usize,mid:usize,last:usize){
    let mut temp = arr.to_vec();
    let temp = &mut temp[..];

    let mut first1 = first;
    let last1 = mid;
    let mut first2 = mid+1;
    let last2 = last;
    let mut index = first1;
    while first1<= last1 && first2<=last2{
        if arr[first1]<arr[first2]{
            temp[index] = arr[first1];
            first1 +=1;
        }

        else {
            temp[index] = arr[first2];
            first2 +=1;
        }
        index +=1;
    }
//  Sao chep not day con 1
    while first1<= last1 {
        temp[index] = arr[first1];
        first1 +=1;
        index +=1;
    }

//  Sao chep not day con 2
    while first2<= last2{
        temp[index] = arr[first2];
        first2 +=1;
        index +=1;
    }

//  Tra ve mang ket qua   
    for index in first..last+1 {
        arr[index] = temp[index]
    }
}


pub fn quick_sort(arr: &mut [f64],left:usize, right:usize) -> &mut[f64] {
    let index_pivot : usize;
    if left < right {
        index_pivot = partition(arr,left,right);
        quick_sort(arr,left,index_pivot-1);
        quick_sort(arr,index_pivot+1,right);
    }
    arr
}
fn partition(arr: &mut [f64],left:usize,right:usize) -> usize {
    let pivot = arr[(left + right)/2];
    let mut i = left; let mut j = right;    
    while i<j {
        while i<=right && arr[i]<pivot{ i+=1;};
        while j>=left && arr[j]>pivot { j -= 1;}
        if i<j {
            arr.swap(i,j);
            i +=1;
            j-=1;
        }
    }
    j
}



pub fn heap_sort(arr: &mut [f64]) -> &mut [f64]{
    let end = arr.len();
    for  start in (0..end/2).rev(){
        sift_down(arr, start, end - 1);
    }
    for end in (1..arr.len()).rev() {
        arr.swap(end, 0);
        sift_down(arr, 0, end - 1);
    }
    arr
}
fn sift_down(arr: &mut [f64],start:usize,end:usize){
    let mut root = start;
    loop {
        let mut child = root * 2 + 1; // Get the left child
        if child > end {
            break;
        }
        if child < end && arr[child] < arr[child + 1] {
            // Right child exists and is greater.
            child += 1;
        }

        if arr[root] < arr[child] {
            // If child is greater than root, swap'em!
            arr.swap(root, child);
            root = child;
        } else {
            break;
        }
    }
}