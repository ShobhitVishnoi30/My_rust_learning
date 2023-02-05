
   pub fn reverse(x: i32) -> i32 {
        let mut rev:i64=0;
        let mut original:i64=x.abs() as i64;
        let mut negative:bool=false;
        if x<0{
            negative=true;
        }
        while original>0{
            let mut temp=original%10;
            if temp !=0{
                rev=rev*10 + temp;
            }else{
                if rev !=0{
                    rev=rev*10 + temp;
                }
            }
            original=original/10;
        }

        if(rev > i32::MAX as i64){
            return 0
        }

        if negative{
            return -(rev as i32)
        }
        return (rev as i32)
   }
