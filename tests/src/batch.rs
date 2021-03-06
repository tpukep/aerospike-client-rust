// Copyright 2015-2017 Aerospike, Inc.
//
// Portions may be licensed to Aerospike, Inc. under one or more contributor
// license agreements.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not
// use this file except in compliance with the License. You may obtain a copy of
// the License at http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
// License for the specific language governing permissions and limitations under
// the License.

use aerospike::BatchRead;
use aerospike::Bins;
use aerospike::{BatchPolicy, WritePolicy, Concurrency};

use env_logger;

use common;

#[test]
fn batch_get() {
    let _ = env_logger::init();

    let client = common::client();
    let namespace: &str = common::namespace();
    let set_name = &common::rand_str(10);
    let mut bpolicy = BatchPolicy::default();
    bpolicy.concurrency = Concurrency::Parallel;
    let wpolicy = WritePolicy::default();

    let bin1 = as_bin!("a", "a value");
    let bin2 = as_bin!("b", "another value");
    let bin3 = as_bin!("c", 42);

    let key1 = as_key!(namespace, set_name, 1);
    client.put(&wpolicy, &key1, &[&bin1, &bin2, &bin3]).unwrap();

    let key2 = as_key!(namespace, set_name, 2);
    client.put(&wpolicy, &key2, &[&bin1, &bin2, &bin3]).unwrap();

    let key3 = as_key!(namespace, set_name, 3);
    client.put(&wpolicy, &key3, &[&bin1, &bin2, &bin3]).unwrap();

    let bins = &["a"];
    let batch = vec![BatchRead::new(key1, Bins::Some(bins)),
                     BatchRead::new(key2, Bins::All),
                     BatchRead::new(key3, Bins::None),
                     BatchRead::new(as_key!(namespace, set_name, -1), Bins::None)];
    let mut results = client.batch_get(&bpolicy, batch).unwrap();

    let record = results.remove(0).record.unwrap();
    assert_eq!(record.bins.keys().count(), 1);
    let record = results.remove(0).record.unwrap();
    assert_eq!(record.bins.keys().count(), 3);
    let record = results.remove(0).record.unwrap();
    assert_eq!(record.bins.keys().count(), 0);
    let record = results.remove(0).record;
    assert!(record.is_none());
}
