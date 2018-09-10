// TODO: split these out into separate files once we add documentation.
//
// Right now, they can all fit in here because they're pretty small.
/// See [http://docs.screeps.com/api/#Game.cpu]
///
/// [http://docs.screeps.com/api/#Game.cpu]: http://docs.screeps.com/api/#Game.cpu
pub mod cpu {
    use std::collections;

    use constants::ReturnCode;

    /// See [v8_getheapstatistics]
    ///
    /// [v8_getheapstatistics]: https://nodejs.org/dist/latest-v8.x/docs/api/v8.html#v8_v8_getheapstatistics
    #[derive(Default, Serialize, Deserialize)]
    pub struct HeapStatistics {
        pub total_heap_size: u32,
        pub total_heap_size_executable: u32,
        pub total_physical_size: u32,
        pub used_heap_size: u32,
        pub heap_size_limit: u32,
        pub malloced_memory: u32,
        pub peak_malloced_memory: u32,
        pub does_zap_garbage: u32,
        pub externally_allocated_size: u32,
    }

    js_serializable!(HeapStatistics);
    js_deserializable!(HeapStatistics);

    /// See [http://docs.screeps.com/api/#Game.cpu]
    ///
    /// [http://docs.screeps.com/api/#Game.cpu]: http://docs.screeps.com/api/#Game.cpu
    pub fn limit() -> f64 {
        js_unwrap!(Game.cpu.limit)
    }

    /// See [http://docs.screeps.com/api/#Game.cpu]
    ///
    /// [http://docs.screeps.com/api/#Game.cpu]: http://docs.screeps.com/api/#Game.cpu
    pub fn tick_limit() -> f64 {
        js_unwrap!(Game.cpu.tickLimit)
    }

    /// See [http://docs.screeps.com/api/#Game.cpu]
    ///
    /// [http://docs.screeps.com/api/#Game.cpu]: http://docs.screeps.com/api/#Game.cpu
    pub fn bucket() -> f64 {
        js_unwrap!(Game.cpu.bucket)
    }

    /// See [http://docs.screeps.com/api/#Game.cpu]
    ///
    /// [http://docs.screeps.com/api/#Game.cpu]: http://docs.screeps.com/api/#Game.cpu
    pub fn shard_limits() -> collections::HashMap<String, f64> {
        js_unwrap!(Game.cpu.shardLimits)
    }

    /// See [http://docs.screeps.com/api/#Game.getHeapStatistics]
    ///
    /// [http://docs.screeps.com/api/#Game.getHeapStatistics]: http://docs.screeps.com/api/#Game.getHeapStatistics
    ///
    /// Returns object with all 0 values if heap statistics are not availe.
    pub fn get_heap_statistics() -> HeapStatistics {
        use stdweb::unstable::TryInto;
        use stdweb::Value;

        let heap_stats: Value =
            js_unwrap!(Game.cpu.getHeapStatistics && Game.cpu.getHeapStatistics());

        match heap_stats {
            Value::Null | Value::Undefined | Value::Bool(false) => HeapStatistics::default(),
            other => other.try_into().expect(
                "expected Game.cpu.getHeapStatistics() to return an object with a known format",
            ),
        }
    }

    /// See [http://docs.screeps.com/api/#Game.getUsed]
    ///
    /// [http://docs.screeps.com/api/#Game.getUsed]: http://docs.screeps.com/api/#Game.getUsed
    pub fn get_used() -> f64 {
        js_unwrap!(Game.cpu.getUsed())
    }

    /// See [http://docs.screeps.com/api/#Game.setShardLimits]
    ///
    /// [http://docs.screeps.com/api/#Game.setShardLimits]: http://docs.screeps.com/api/#Game.setShardLimits
    pub fn set_shard_limits(limits: collections::HashMap<String, f64>) -> ReturnCode {
        js_unwrap!(Game.cpu.setShardLimits(@{limits}))
    }
}

/// See [http://docs.screeps.com/api/#Game.gcl]
///
/// [http://docs.screeps.com/api/#Game.gcl]: http://docs.screeps.com/api/#Game.gcl
pub mod gcl {
    /// See [http://docs.screeps.com/api/#Game.gcl]
    ///
    /// [http://docs.screeps.com/api/#Game.gcl]: http://docs.screeps.com/api/#Game.gcl
    pub fn level() -> u32 {
        js_unwrap!(Game.gcl.level)
    }

    /// See [http://docs.screeps.com/api/#Game.gcl]
    ///
    /// [http://docs.screeps.com/api/#Game.gcl]: http://docs.screeps.com/api/#Game.gcl
    pub fn progress() -> f64 {
        js_unwrap!(Game.gcl.progress)
    }

    /// See [http://docs.screeps.com/api/#Game.gcl]
    ///
    /// [http://docs.screeps.com/api/#Game.gcl]: http://docs.screeps.com/api/#Game.gcl
    pub fn progress_total() -> f64 {
        js_unwrap!(Game.gcl.progressTotal)
    }
}

/// See [http://docs.screeps.com/api/#Game.map]
///
/// [http://docs.screeps.com/api/#Game.map]: http://docs.screeps.com/api/#Game.map
pub mod map {
    use std::collections;
    use stdweb::unstable::{TryInto, TryFrom};

    use {Direction, RoomPosition, Terrain, Room};
    use constants::{ReturnCode, find::Exit};

    /// See [http://docs.screeps.com/api/#Game.map.describeExits]
    ///
    /// [http://docs.screeps.com/api/#Game.map.describeExits]: http://docs.screeps.com/api/#Game.map.describeExits
    pub fn describe_exits(room_name: &str) -> collections::HashMap<Direction, String> {
        use num_traits::FromPrimitive;

        let orig: collections::HashMap<String, String> =
            js_unwrap!(Game.map.describeExits(@{room_name}));

        orig.into_iter()
            .map(|(key, value)| {
                let key: u32 = key.parse().expect(
                    "expected all directions returned from Game.map.describeExits to be integers",
                );
                (
                Direction::from_u32(key).expect("expected all directions returned from Game.map.describeExits to be directions"),
                value,
            )
            })
            .collect()
    }

    /// See [http://docs.screeps.com/api/#Game.map.getRoomLinearDistance]
    ///
    /// [http://docs.screeps.com/api/#Game.map.getRoomLinearDistance]: http://docs.screeps.com/api/#Game.map.getRoomLinearDistance
    pub fn get_room_linear_distance(room1: &str, room2: &str, continuous: bool) -> u32 {
        js_unwrap!(Game.map.getRoomLinearDistance(@{room1}, @{room2}, @{continuous}))
    }

    /// See [http://docs.screeps.com/api/#Game.map.getTerrainAt]
    ///
    /// [http://docs.screeps.com/api/#Game.map.getTerrainAt]: http://docs.screeps.com/api/#Game.map.getTerrainAt
    pub fn get_terrain_at(pos: &RoomPosition) -> Terrain {
        js_unwrap!(__terrain_type_str_to_num(Game.map.getTerrainAt(@{pos.as_ref()})))
    }

    /// See [http://docs.screeps.com/api/#Game.map.getWorldSize]
    ///
    /// [http://docs.screeps.com/api/#Game.map.getWorldSize]: http://docs.screeps.com/api/#Game.map.getWorldSize
    pub fn get_world_size() -> u32 {
        js_unwrap!(Game.map.getWorldSize())
    }

    /// See [http://docs.screeps.com/api/#Game.map.isRoomAvailable]
    ///
    /// [http://docs.screeps.com/api/#Game.map.isRoomAvaile]: http://docs.screeps.com/api/#Game.map.isRoomAvaile
    pub fn is_room_available(room_name: &str) -> bool {
        js_unwrap!(Game.map.isRoomAvailable(@{room_name}))
    }

    /// Implements `Game.map.findExit`.
    /// 
    /// Does not yet support callbacks.
    pub fn find_exit(from_room: Room, to_room: Room) -> Result<Exit, ReturnCode> {
        let code: i32 = js_unwrap!{Game.map.findExit(@{from_room.name()}, @{to_room.name()})};
        Exit::try_from(code).map_err(|v| v.try_into().expect("find_exit: Error code not recognized."))
    }

    #[allow(unused_variables)]
    pub fn find_route(from_room: Room, to_room: Room, route_callback: Option<impl Fn(&str, &str) -> u32>) -> !{
        unimplemented!()
    }
}

pub mod market {
    use {Room};
    use constants::{ReturnCode, ResourceType};

    pub enum OrderType {
        Sell,
        Buy
    }

    impl OrderType {
        pub fn as_string(&self) -> String {
            match self {
                OrderType::Sell => String::from("sell"),
                OrderType::Buy => String::from("buy")
            }
        }
    }

    pub fn credits() -> u32 {
        js_unwrap!(Game.market.credits)
    }

    pub fn incoming_transactions() -> !{
        unimplemented!()
    }

    pub fn outgoing_transactions() -> !{
        unimplemented!()
    }

    pub fn orders() -> !{
        unimplemented!()
    }

    pub fn calc_transaction_cost(amount: u32, room1: &Room, room2: &Room) -> u32 {
        js_unwrap!(Game.market.calcTransactionCost(@{amount}, @{room1.name()}, @{room2.name()}))
    }

    pub fn cancel_order(order_id: &str) -> ReturnCode {
        js_unwrap!(Game.market.cancelOrder(@{order_id}))
    }

    pub fn change_order_price(order_id: &str, new_price: u32) -> ReturnCode {
        js_unwrap!(Game.market.changeOrderPrice(@{order_id}, @{new_price}))
    }

    pub fn create_order(order_type: OrderType, resource_type: ResourceType, 
                        price: f64, total_amount: u32, room: &Room) -> ReturnCode {
        js_unwrap!{
            Game.market.createOrder(@{order_type.as_string()},
                                    __resource_type_num_to_str(@{resource_type as i32}),
                                    @{price},
                                    @{total_amount},
                                    @{room.name()})
        }
    }

    pub fn deal(order_id: &str, amount: u32, target_room: Room) -> ReturnCode {
        js_unwrap!{Game.market.deal(@{order_id}, @{amount}, @{target_room.name()})}
    }

    pub fn extend_order(order_id: &str, add_amount: u32) -> ReturnCode {
        js_unwrap!{Game.market.extendOrder(@{order_id}, @{add_amount})}
    }

    pub fn get_all_orders() -> ! {
        unimplemented!()
    }

    pub fn get_order() -> ! {
        unimplemented!()
    }
}


/// See [http://docs.screeps.com/api/#Game.shard]
///
/// [http://docs.screeps.com/api/#Game.shard]: http://docs.screeps.com/api/#Game.shard
pub mod shard {
    /// See [http://docs.screeps.com/api/#Game.shard]
    ///
    /// [http://docs.screeps.com/api/#Game.shard]: http://docs.screeps.com/api/#Game.shard
    pub fn name() -> String {
        js_unwrap!(Game.shard.name)
    }

    /// See [http://docs.screeps.com/api/#Game.shard]
    ///
    /// [http://docs.screeps.com/api/#Game.shard]: http://docs.screeps.com/api/#Game.shard
    pub fn shard_type() -> String {
        js_unwrap!(Game.shard.type)
    }

    /// See [http://docs.screeps.com/api/#Game.shard]
    ///
    /// [http://docs.screeps.com/api/#Game.shard]: http://docs.screeps.com/api/#Game.shard
    pub fn ptr() -> bool {
        js_unwrap!(Game.shard.ptr)
    }
}

game_map_access! {
    /// See [http://docs.screeps.com/api/#Game.constructionSites]
    ///
    /// [http://docs.screeps.com/api/#Game.constructionSites]: http://docs.screeps.com/api/#Game.constructionSites
    (construction_sites, objects::ConstructionSite, Game.constructionSites),
    /// See [http://docs.screeps.com/api/#Game.creeps]
    ///
    /// [http://docs.screeps.com/api/#Game.creeps]: http://docs.screeps.com/api/#Game.creeps
    (creeps, objects::Creep, Game.creeps),
    /// See [http://docs.screeps.com/api/#Game.flags]
    ///
    /// [http://docs.screeps.com/api/#Game.flags]: http://docs.screeps.com/api/#Game.flags
    (flags, objects::Flag, Game.flags),
    // TODO: See [http://docs.screeps.com/api/#Game.resources]
    ///
    /// [http://docs.screeps.com/api/#Game.resources]: http://docs.screeps.com/api/#Game.resources
    /// See [http://docs.screeps.com/api/#Game.rooms]
    ///
    /// [http://docs.screeps.com/api/#Game.rooms]: http://docs.screeps.com/api/#Game.rooms
    (rooms, objects::Room, Game.rooms),
    /// See [http://docs.screeps.com/api/#Game.spawns]
    ///
    /// [http://docs.screeps.com/api/#Game.spawns]: http://docs.screeps.com/api/#Game.spawns
    (spawns, objects::StructureSpawn, Game.spawns),
    /// See [http://docs.screeps.com/api/#Game.structures]
    ///
    /// [http://docs.screeps.com/api/#Game.structures]: http://docs.screeps.com/api/#Game.structures
    (structures, objects::Structure, Game.structures)
}

/// See [http://docs.screeps.com/api/#Game.time]
///
/// [http://docs.screeps.com/api/#Game.time]: http://docs.screeps.com/api/#Game.time
pub fn time() -> u32 {
    js_unwrap!(Game.time)
}

/// See [http://docs.screeps.com/api/#Game.getObjectById]
///
/// [http://docs.screeps.com/api/#Game.getObjectById]: http://docs.screeps.com/api/#Game.getObjectById
pub fn get_object(id: &str) -> Option<::objects::RoomObject> {
    js_unwrap!(Game.getObjectById(@{id}))
}

pub fn notify(message: &str, group_interval: Option<u32>) {
    js!{Game.notify(@{message}, @{group_interval.unwrap_or(0)})};
}
