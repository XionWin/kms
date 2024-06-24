use crate::{util, Drm};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Property {
    pub(crate) handle: *const crate::DrmModeProperty,
    pub(crate) prop_id: libc::c_uint,
    pub(crate) flags: libc::c_uint,
    pub(crate) name: String,
    pub(crate) count_values: libc::c_int,
    pub(crate) values: Vec<libc::c_ulong>,
    pub(crate) count_enums: libc::c_int,
    pub(crate) enums: Vec<PropertyEnum>,
}

impl Drop for Property {
    fn drop(&mut self) {
        unsafe { crate::drmModeFreeProperty(self.handle) }
    }
}

fn get_values_from_ptr<T>(ptr: *const T, len: usize) -> Vec<T>
where
    T: Copy,
{
    let mut result = Vec::new();
    for index in 0..len {
        let cur_ptr = unsafe { ptr.add(index) };
        let v = unsafe { *cur_ptr };
        result.push(v);
    }
    result
}


impl Into<Property> for *const crate::DrmModeProperty {
    fn into(self) -> Property {
        let v = unsafe { *self };
        Property {
            handle: self,
            prop_id: v.prop_id,
            flags: v.flags,
            name: util::get_string_from_chars(v.name.as_slice()),
            count_values: v.count_values,
            values: get_values_from_ptr(v.values, v.count_values as _),
            count_enums: v.count_enums,
            enums: get_values_from_ptr(v.enums, v.count_enums as _)
                .into_iter()
                .map(|x| x.into())
                .collect(),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct PropertyEnum {
    pub(crate) value: libc::c_ulong,
    pub(crate) name: String,
}

impl Into<PropertyEnum> for crate::DrmModePropertyEnum {
    fn into(self) -> PropertyEnum {
        PropertyEnum {
            value: self.value,
            name: util::get_string_from_chars(self.name.as_slice()),
        }
    }
}

impl Drm {
    pub fn get_property(&self, property_id: libc::c_uint) -> Property {
        unsafe { crate::drmModeGetProperty(self.fd, property_id) }.into()
    }
}
