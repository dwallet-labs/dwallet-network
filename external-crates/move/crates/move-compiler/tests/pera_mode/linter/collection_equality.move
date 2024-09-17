// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module a::test {
    use pera::bag::Bag;
    use pera::object_bag::ObjectBag;
    use pera::table::Table;
    use pera::object_table::ObjectTable;
    use pera::linked_table::LinkedTable;
    use pera::table_vec::TableVec;
    use pera::vec_map::VecMap;
    use pera::vec_set::VecSet;



    public fun bag_eq(bag1: &Bag, bag2: &Bag): bool {
        bag1 == bag2
    }

    public fun obj_bag_neq(bag1: &ObjectBag, bag2: &ObjectBag): bool {
        bag1 != bag2
    }

    public fun table_eq(table1: &Table<u64, u64>, table2: &Table<u64, u64>): bool {
        table1 == table2
    }

    public fun obj_table_eq<K: copy + drop + store, V: key + store>(
        table1: &ObjectTable<K, V>,
        table2: &ObjectTable<K, V>
    ): bool {
            table1 == table2
    }

    public fun linked_table_neq(table1: &LinkedTable<u64, u64>, table2: &LinkedTable<u64, u64>): bool {
        table1 == table2
    }

    public fun table_vec_eq(table1: &TableVec<u64>, table2: &TableVec<u64>): bool {
        table1 == table2
    }

    public fun vec_map_eq(vec1: &VecMap<u64, u64>, vec2: &VecMap<u64, u64>): bool {
        vec1 == vec2
    }

    public fun vec_set_eq(vec1: &VecSet<u64>, vec2: &VecSet<u64>): bool {
        vec1 == vec2
    }
}

module pera::object {
    struct UID has store {
        id: address,
    }
}

module pera::bag {
    use pera::object::UID;

    struct Bag has key, store {
        id: UID
    }
}

module pera::object_bag {
    use pera::object::UID;

    struct ObjectBag has key, store {
        id: UID
    }
}

module pera::table {
    use pera::object::UID;

    struct Table<phantom K: copy + drop + store, phantom V: store> has key, store {
        id: UID
    }
}

module pera::object_table {
    use pera::object::UID;

    struct ObjectTable<phantom K: copy + drop + store, phantom V: key + store> has key, store {
        id: UID
    }
}

module pera::linked_table {
    use pera::object::UID;

    struct LinkedTable<phantom K: copy + drop + store, phantom V: store> has key, store {
        id: UID
    }
}

module pera::table_vec {
    use pera::object::UID;

    struct TableVec<phantom Element: store> has key, store {
        id: UID
    }
}

module pera::vec_map {
    use pera::object::UID;

    struct VecMap<phantom K: copy, phantom V> has key, store {
        id: UID
    }
}

module pera::vec_set {
    use pera::object::UID;

    struct VecSet<phantom K: copy + drop> has key, store {
        id: UID
    }
}
