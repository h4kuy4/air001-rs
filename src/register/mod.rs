#[repr(transparent)]
pub struct RW<T> 
where T: Copy {
    reg: T,
}

impl<T> RW<T>
where T: Copy {
    pub fn read(&self) -> T {
        return self.reg;
    }

    pub fn write(&mut self, val: T) {
        self.reg = val; 
    }
}

#[repr(transparent)]
pub struct RO<T> 
where T: Copy {
    reg: T,
}

impl<T> RO<T>
where T: Copy {
    pub fn read(&self) -> T {
        return self.reg;
    }

    pub unsafe fn write(&mut self, val: T) {
        self.reg = val; 
    }
}

#[repr(transparent)]
pub struct WO<T> 
where T: Copy {
    reg: T,
}

impl<T> WO<T>
where T: Copy {
    pub fn write(&mut self, val: T) {
        self.reg = val; 
    }
}
