// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module a::test {
    use dwallet::bag::Bag;
    use dwallet::object_bag::ObjectBag;
    use dwallet::table::Table;
    use dwallet::object_table::ObjectTable;
    use dwallet::linked_table::LinkedTable;
    use dwallet::table_vec::TableVec;
    use dwallet::vec_map::VecMap;
    use dwallet::vec_set::VecSet;



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

module dwallet::object {
    struct UID has store {
        id: address,
    }
}

module dwallet::bag {
    use dwallet::object::UID;

    struct Bag has key, store {
        id: UID
    }
}

module dwallet::object_bag {
    use dwallet::object::UID;

    struct ObjectBag has key, store {
        id: UID
    }
}

module dwallet::table {
    use dwallet::object::UID;

    struct Table<phantom K: copy + drop + store, phantom V: store> has key, store {
        id: UID
    }
}

module dwallet::object_table {
    use dwallet::object::UID;

    struct ObjectTable<phantom K: copy + drop + store, phantom V: key + store> has key, store {
        id: UID
    }
}

module dwallet::linked_table {
    use dwallet::object::UID;

    struct LinkedTable<phantom K: copy + drop + store, phantom V: store> has key, store {
        id: UID
    }
}

module dwallet::table_vec {
    use dwallet::object::UID;

    struct TableVec<phantom Element: store> has key, store {
        id: UID
    }
}

module dwallet::vec_map {
    use dwallet::object::UID;

    struct VecMap<phantom K: copy, phantom V> has key, store {
        id: UID
    }
}

module dwallet::vec_set {
    use dwallet::object::UID;

    struct VecSet<phantom K: copy + drop> has key, store {
        id: UID
    }
}
