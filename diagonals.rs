fn diagonals(matrix: &[Vec<i8>] ) -> Vec<Vec<i8>> {
    let mut diags : Vec<Vec<i8>> = vec![];
    
    // check for empty matrix
    if matrix.len() == 0 || matrix[0].len() == 0 {
        return diags;
    }
    
    let dims = (matrix.len(), matrix[0].len());
    
    // check for single element matrix
    if dims.0 == 1 && dims.1 == 1 {
        diags.push(vec![matrix[0][0]]);
        return diags;
    }
    
    let mut indices = (dims.0 - 1, 0);

    // find normal diagonals
    loop {
        let mut cur_indices = indices.clone();
        let mut diag : Vec<i8> = vec![];
        while cur_indices.0 < dims.0 && cur_indices.1 < dims.1 {
            diag.push(matrix[cur_indices.0][cur_indices.1]);
            cur_indices.0 += 1;
            cur_indices.1 += 1;
        }
        
        diags.push(diag);
        
        if indices.0 > 0 {
            indices.0 -= 1;
        } else if indices.1 + 1 < dims.1 {
            indices.1 += 1;
        } else {
            break;
        }
    }
    
    // find reverse diagonals
    indices = (dims.0 - 1, dims.1 - 1);
    loop {
        let mut cur_indices = indices.clone();
        let mut diag : Vec<i8> = vec![];
        while cur_indices.0 < dims.0 {
            diag.push(matrix[cur_indices.0][cur_indices.1]);
            cur_indices.0 += 1;
            if cur_indices.1 == 0 {
                break;
            }
            cur_indices.1 -= 1;
        }
        
        diags.push(diag);
        
        if indices.0 > 0 {
            indices.0 -= 1;
        } else if indices.1 > 0 {
            indices.1 -= 1;
        } else {
            break;
        }
    }
    
    diags
}
