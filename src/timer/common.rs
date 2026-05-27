use std::marker::PhantomData;

#[derive(Default)]
pub struct Year<T>(i32, PhantomData<T>);
#[derive(Default)]
pub struct Month<T>(u8, PhantomData<T>);
#[derive(Default)]
pub struct Day<T>(u8, PhantomData<T>);
#[derive(Default)]
pub struct Date<T>(i32, PhantomData<T>);
#[derive(Default)]
pub struct DateFrac<T>(u16, PhantomData<T>);

/**
 * Data structure to convert between Date and triplet (year, month, and day).
 * @see ConvertDateToYMD(), ConvertYMDToDate()
 */
#[derive(Default)]
pub struct YearMonthDay<T> {
    ///< Year (0...)
    pub year: Year<T>,
    ///< Month (0..11)
    pub month: Month<T>,
    ///< Day (1..31)
    pub day: Day<T>,
}
