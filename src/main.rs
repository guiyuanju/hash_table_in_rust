mod bench;
mod generator;
mod table;

use bench::*;
use generator::*;
use table::*;

const READ_FACTOR: usize = 200;
const DELETE_FACTOR: f64 = 0.1;

fn main() {
    let mut data: Vec<(f32, f32)> = vec![];
    for i in 10..1500 {
        let average = bench_average_op_time(i);
        data.push((i as f32, average));
        println!(
            "scale: {:?}, total: {:?}, average: {:?}",
            i,
            average * i as f32,
            average
        );
    }
    _ = draw("Op Time", &data);

    let y_avgs = running_averages(&data.iter().map(|p| p.1).collect::<Vec<f32>>());
    let data = data
        .iter()
        .map(|p| p.0)
        .zip(y_avgs)
        .collect::<Vec<(f32, f32)>>();
    _ = draw("Amortized Op Time", &data)
}

fn running_averages(xs: &[f32]) -> Vec<f32> {
    let mut running_averages: Vec<f32> = vec![];
    let mut sum = 0.0f32;

    for (i, v) in xs.iter().enumerate() {
        sum += v;
        running_averages.push(sum / (i + 1) as f32);
    }

    running_averages
}

fn bench_average_op_time(scale: usize) -> f32 {
    let kvs = generate_set_kvs(scale);
    let keys = generate_get_keys(scale, &kvs, READ_FACTOR);
    let delete_keys = generate_delete_keys(scale, &kvs);
    let total = bench::measure(|| {
        let mut table = Table::new();
        for (k, v) in kvs {
            table.set(&k, &v);
        }
        for k in delete_keys {
            table.delete(&k);
        }
        for k in keys {
            table.get(&k);
        }
    });
    total.as_secs_f32() / scale as f32
}

// generate random key value pairs to be inserted
fn generate_set_kvs(scale: usize) -> Vec<(Key, Value)> {
    let generator = StringGenerator::new();
    generator
        .take(scale)
        .map(|s| (Key::new(&s), Value::String(s)))
        .collect::<Vec<(Key, Value)>>()
}

// generate random keys, half are known to exist in table
fn generate_get_keys(scale: usize, existing_kvs: &[(Key, Value)], repeat: usize) -> Vec<Key> {
    let mut keys = StringGenerator::new()
        .take(scale)
        .map(|s| Key::new(&s))
        .collect::<Vec<Key>>();
    for i in (0..scale).step_by(2) {
        keys[i] = existing_kvs[i].0.clone();
    }
    keys.iter()
        .cycle()
        .take(scale * repeat)
        .cloned()
        .collect::<Vec<Key>>()
}

fn generate_delete_keys(scale: usize, existing_kvs: &[(Key, Value)]) -> Vec<Key> {
    let mut keys: Vec<Key> = vec![];
    let mut count = (scale as f64 * DELETE_FACTOR) as usize;
    if count <= 1 {
        count = 2
    }
    for (i, kv) in existing_kvs.iter().enumerate() {
        if i % count == 0 {
            keys.push(kv.0.clone())
        }
    }
    keys
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_general() {
        let mut table = Table::new();
        table.set(&Key::new("key1"), &Value::Boolean(false));
        table.set(&Key::new("key2"), &Value::Number(10.0));
        table.set(&Key::new("key3"), &Value::String("hi".to_string()));
        assert!(table.get(&Key::new("key1")).unwrap() == Value::Boolean(false));
        assert!(table.get(&Key::new("key2")).unwrap() == Value::Number(10.0));
        assert!(table.get(&Key::new("key3")).unwrap() == Value::String("hi".to_string()));
        assert!(table.get(&Key::new("key4")).is_none());
        table.delete(&Key::new("key1"));
        assert!(table.get(&Key::new("key1")).is_none());
        table.delete(&Key::new("key4"));
        assert!(table.get(&Key::new("key4")).is_none());
    }

    #[test]
    fn test_set_get() {
        let mut table = Table::new();
        let generator = StringGenerator::new();

        generator.take(1000).for_each(|s| {
            // println!(
            //     "size = {:}, load factor = {:}, key = {:}",
            //     table.capacity(),
            //     table.load_factor(),
            //     s
            // );
            let key = Key::new(&s);
            let value = Value::String(s);
            table.set(&key, &value);
            assert!(table.get(&key).unwrap() == value);
        })
    }

    #[test]
    fn test_delete() {
        let mut table = Table::new();
        let generator = StringGenerator::new();

        let keys = generator
            .take(1000)
            .map(|s| Key::new(&s))
            .collect::<Vec<Key>>();

        for k in &keys {
            table.set(k, &Value::Number(0.0));
            println!(
                "count = {:?}, capacipty = {:?}, load_factor = {:?}",
                table.count,
                table.capacity(),
                table.load_factor()
            )
        }

        for (i, k) in keys.iter().enumerate() {
            if i % 2 == 0 {
                table.delete(&k);
                println!(
                    "count = {:?}, capacipty = {:?}, load_factor = {:?}",
                    table.count,
                    table.capacity(),
                    table.load_factor()
                )
            }
        }

        for (i, k) in keys.iter().enumerate() {
            if i % 2 == 0 {
                assert!(table.get(k).is_none());
            } else {
                assert!(table.get(k).is_some());
            }
        }

        let generator = StringGenerator::new();
        let keys = generator
            .take(500)
            .map(|s| Key::new(&s))
            .collect::<Vec<Key>>();

        for k in &keys {
            table.set(k, &Value::Number(0.0));
            println!(
                "count = {:?}, capacipty = {:?}, load_factor = {:?}",
                table.count,
                table.capacity(),
                table.load_factor()
            )
        }
    }
}
