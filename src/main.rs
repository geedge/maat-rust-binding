pub mod bindings;

use crate::bindings::MAAT_SCAN_HIT;
use std::ffi::CString;

const ARRAY_SIZE: usize = 10;

pub fn maat_hs_string_scan() {
    let redis_ip = CString::new("127.0.0.1").unwrap();
    let redis_port = 6379;
    let redis_db = 0;
    let log_path = CString::new("./maat_framework_gtest.log").unwrap();
    let log_level = bindings::LOG_LEVEL_INFO;
    let table_info_path = CString::new("./table_info.conf").unwrap();

    unsafe {
        let opts = bindings::maat_options_new();
        bindings::maat_options_set_redis(opts, redis_ip.as_ptr(), redis_port, redis_db);
        bindings::maat_options_set_logger(opts, log_path.as_ptr(), log_level);

        let maat_inst = bindings::maat_new(opts, table_info_path.as_ptr());
        bindings::maat_options_free(opts);

        let table_name = CString::new("HTTP_URL").unwrap();
        let table_id = bindings::maat_get_table_id(maat_inst, table_name.as_ptr());
        assert_ne!(table_id, 0);

        let mut results: [bindings::CLongLong; ARRAY_SIZE] = [-1; ARRAY_SIZE];
        let mut n_hit_result: usize = 0;
        let thread_id = 0;
        let state = bindings::maat_state_new(maat_inst, thread_id);
        let scan_data = CString::new("http://www.cyberessays.com/search_results.php?action=search&query=username,abckkk,1234567").unwrap();

        let n_hit_result_ptr = &mut n_hit_result as *mut usize;

        let ret = bindings::maat_scan_string(
            maat_inst,
            table_id,
            scan_data.as_ptr(),
            scan_data.as_bytes().len(),
            results.as_mut_ptr(),
            ARRAY_SIZE,
            n_hit_result_ptr,
            state,
        );

        assert_eq!(ret, MAAT_SCAN_HIT);
        assert_eq!(n_hit_result, 1);
        assert_eq!(results[0], 125);
        bindings::maat_state_free(state);
        println!("hit compile_id:{}", results[0]);
    }
}
fn main() {
    crate::maat_hs_string_scan();
}
