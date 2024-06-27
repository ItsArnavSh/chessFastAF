pub fn map_to_num(bitmap:u64)->u8
{
    let mut bitmap = bitmap;
    for i in 1..65
    {
        if (bitmap&1)==1
                {return i;}
            bitmap=bitmap>>1;
    }
    0
}