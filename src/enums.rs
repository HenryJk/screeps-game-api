use enum_dispatch::enum_dispatch;
use wasm_bindgen::{JsCast, JsValue};

use crate::objects::*;
use crate::prelude::*;

#[enum_dispatch(Attackable)]
pub enum AttackableObject {
    Creep,
    PowerCreep,
    StructureContainer,
    StructureExtension,
    StructureExtractor,
    StructureFactory,
    StructureInvaderCore,
    StructureLab,
    StructureLink,
    StructureNuker,
    StructureObserver,
    StructurePowerBank,
    StructurePowerSpawn,
    StructureRampart,
    StructureRoad,
    StructureSpawn,
    StructureStorage,
    StructureTerminal,
    StructureTower,
    StructureWall,
}

impl From<AttackableObject> for RoomObject {
    fn from(attackable: AttackableObject) -> Self {
        use AttackableObject::*;

        match attackable {
            Creep(o) => RoomObject::from(o),
            PowerCreep(o) => RoomObject::from(o),
            StructureContainer(o) => RoomObject::from(o),
            StructureExtension(o) => RoomObject::from(o),
            StructureExtractor(o) => RoomObject::from(o),
            StructureFactory(o) => RoomObject::from(o),
            StructureInvaderCore(o) => RoomObject::from(o),
            StructureLab(o) => RoomObject::from(o),
            StructureLink(o) => RoomObject::from(o),
            StructureNuker(o) => RoomObject::from(o),
            StructureObserver(o) => RoomObject::from(o),
            StructurePowerBank(o) => RoomObject::from(o),
            StructurePowerSpawn(o) => RoomObject::from(o),
            StructureRampart(o) => RoomObject::from(o),
            StructureRoad(o) => RoomObject::from(o),
            StructureSpawn(o) => RoomObject::from(o),
            StructureStorage(o) => RoomObject::from(o),
            StructureTerminal(o) => RoomObject::from(o),
            StructureTower(o) => RoomObject::from(o),
            StructureWall(o) => RoomObject::from(o),
        }
    }
}

impl AsRef<RoomObject> for AttackableObject {
    fn as_ref(&self) -> &RoomObject {
        use AttackableObject::*;

        match self {
            Creep(o) => o.as_ref(),
            PowerCreep(o) => o.as_ref(),
            StructureContainer(o) => o.as_ref(),
            StructureExtension(o) => o.as_ref(),
            StructureExtractor(o) => o.as_ref(),
            StructureFactory(o) => o.as_ref(),
            StructureInvaderCore(o) => o.as_ref(),
            StructureLab(o) => o.as_ref(),
            StructureLink(o) => o.as_ref(),
            StructureNuker(o) => o.as_ref(),
            StructureObserver(o) => o.as_ref(),
            StructurePowerBank(o) => o.as_ref(),
            StructurePowerSpawn(o) => o.as_ref(),
            StructureRampart(o) => o.as_ref(),
            StructureRoad(o) => o.as_ref(),
            StructureSpawn(o) => o.as_ref(),
            StructureStorage(o) => o.as_ref(),
            StructureTerminal(o) => o.as_ref(),
            StructureTower(o) => o.as_ref(),
            StructureWall(o) => o.as_ref(),
        }
    }
}

#[enum_dispatch(CanDecay)]
pub enum DecayingObject {
    Deposit,
    Ruin,
    #[cfg(feature = "enable-score")]
    #[cfg_attr(docsrs, doc(cfg(feature = "enable-score")))]
    ScoreContainer,
    StructureContainer,
    StructurePortal,
    StructurePowerBank,
    StructureRampart,
    StructureRoad,
    Tombstone,
}

#[enum_dispatch(HasCooldown)]
pub enum CooldownObject {
    Deposit,
    StructureExtractor,
    StructureFactory,
    StructureLab,
    StructureLink,
    StructureNuker,
    StructureTerminal,
}

#[enum_dispatch(HasId)]
pub enum ObjectWithId {
    Deposit,
    Mineral,
    Nuke,
    Resource,
    Ruin,
    #[cfg(feature = "enable-score")]
    #[cfg_attr(docsrs, doc(cfg(feature = "enable-score")))]
    ScoreCollector,
    #[cfg(feature = "enable-score")]
    #[cfg_attr(docsrs, doc(cfg(feature = "enable-score")))]
    ScoreContainer,
    Source,
    StructureContainer,
    StructureController,
    StructureExtension,
    StructureExtractor,
    StructureFactory,
    StructureInvaderCore,
    StructureKeeperLair,
    StructureLab,
    StructureLink,
    StructureNuker,
    StructureObserver,
    StructurePortal,
    StructurePowerBank,
    StructurePowerSpawn,
    StructureRampart,
    StructureRoad,
    StructureSpawn,
    StructureStorage,
    StructureTerminal,
    StructureTower,
    StructureWall,
    Tombstone,
}

#[enum_dispatch(MaybeHasId)]
pub enum ObjectWithMaybeId {
    ConstructionSite,
    Creep,
    Deposit,
    Mineral,
    Nuke,
    PowerCreep,
    Resource,
    Ruin,
    #[cfg(feature = "enable-score")]
    #[cfg_attr(docsrs, doc(cfg(feature = "enable-score")))]
    ScoreCollector,
    #[cfg(feature = "enable-score")]
    #[cfg_attr(docsrs, doc(cfg(feature = "enable-score")))]
    ScoreContainer,
    Source,
    StructureContainer,
    StructureController,
    StructureExtension,
    StructureExtractor,
    StructureFactory,
    StructureInvaderCore,
    StructureKeeperLair,
    StructureLab,
    StructureLink,
    StructureNuker,
    StructureObserver,
    StructurePortal,
    StructurePowerBank,
    StructurePowerSpawn,
    StructureRampart,
    StructureRoad,
    StructureSpawn,
    StructureStorage,
    StructureTerminal,
    StructureTower,
    StructureWall,
    Tombstone,
}

#[enum_dispatch(HasPosition)]
pub enum ObjectWithPosition {
    ConstructionSite,
    Creep,
    Deposit,
    Flag,
    Mineral,
    Nuke,
    PowerCreep,
    Resource,
    RoomPosition,
    Ruin,
    #[cfg(feature = "enable-score")]
    #[cfg_attr(docsrs, doc(cfg(feature = "enable-score")))]
    ScoreCollector,
    #[cfg(feature = "enable-score")]
    #[cfg_attr(docsrs, doc(cfg(feature = "enable-score")))]
    ScoreContainer,
    Source,
    StructureContainer,
    StructureController,
    StructureExtension,
    StructureExtractor,
    StructureFactory,
    StructureInvaderCore,
    StructureKeeperLair,
    StructureLab,
    StructureLink,
    StructureNuker,
    StructureObserver,
    StructurePortal,
    StructurePowerBank,
    StructurePowerSpawn,
    StructureRampart,
    StructureRoad,
    StructureSpawn,
    StructureStorage,
    StructureTerminal,
    StructureTower,
    StructureWall,
    Tombstone,
}

#[enum_dispatch(HasStore)]
pub enum StoreObject {
    Creep,
    PowerCreep,
    Ruin,
    #[cfg(feature = "enable-score")]
    #[cfg_attr(docsrs, doc(cfg(feature = "enable-score")))]
    ScoreCollector,
    #[cfg(feature = "enable-score")]
    #[cfg_attr(docsrs, doc(cfg(feature = "enable-score")))]
    ScoreContainer,
    StructureContainer,
    StructureExtension,
    StructureFactory,
    StructureLab,
    StructureLink,
    StructureNuker,
    StructurePowerSpawn,
    StructureSpawn,
    StructureStorage,
    StructureTerminal,
    StructureTower,
    Tombstone,
}

/// Enum used for converting a [`Structure`] into a typed object of its specific
/// structure type.
#[enum_dispatch(OwnedStructureProperties)]
pub enum OwnedStructureObject {
    StructureController,
    StructureExtension,
    StructureExtractor,
    StructureFactory,
    StructureInvaderCore,
    StructureKeeperLair,
    StructureLab,
    StructureLink,
    StructureNuker,
    StructureObserver,
    StructurePowerSpawn,
    StructureRampart,
    StructureSpawn,
    StructureStorage,
    StructureTerminal,
    StructureTower,
}

// todo TryFrom<Structure> for OwnedStructureObject

/// Any enum representing any game object that inherits the [`RoomObject`] type.
#[enum_dispatch(RoomObjectProperties)]
pub enum TypedRoomObject {
    ConstructionSite,
    Creep,
    Deposit,
    Flag,
    Mineral,
    Nuke,
    PowerCreep,
    Resource,
    Ruin,
    #[cfg(feature = "enable-score")]
    #[cfg_attr(docsrs, doc(cfg(feature = "enable-score")))]
    ScoreCollector,
    #[cfg(feature = "enable-score")]
    #[cfg_attr(docsrs, doc(cfg(feature = "enable-score")))]
    ScoreContainer,
    Source,
    StructureContainer,
    StructureController,
    StructureExtension,
    StructureExtractor,
    StructureFactory,
    StructureInvaderCore,
    StructureKeeperLair,
    StructureLab,
    StructureLink,
    StructureNuker,
    StructureObserver,
    StructurePortal,
    StructurePowerBank,
    StructurePowerSpawn,
    StructureRampart,
    StructureRoad,
    StructureSpawn,
    StructureStorage,
    StructureTerminal,
    StructureTower,
    StructureWall,
    Tombstone,
}

#[enum_dispatch(SharedCreepProperties)]
pub enum MovableObject {
    Creep,
    PowerCreep,
}

/// Enum used for converting a [`Structure`] into a typed object of its specific
/// structure type.
#[enum_dispatch(StructureProperties, HasPosition)]
pub enum StructureObject {
    StructureContainer,
    StructureController,
    StructureExtension,
    StructureExtractor,
    StructureFactory,
    StructureInvaderCore,
    StructureKeeperLair,
    StructureLab,
    StructureLink,
    StructureNuker,
    StructureObserver,
    StructurePortal,
    StructurePowerBank,
    StructurePowerSpawn,
    StructureRampart,
    StructureRoad,
    StructureSpawn,
    StructureStorage,
    StructureTerminal,
    StructureTower,
    StructureWall,
}

impl From<JsValue> for StructureObject {
    fn from(reference: JsValue) -> Self {
        let structure: Structure = reference.unchecked_into();

        structure.into()
    }
}

impl From<Structure> for StructureObject {
    fn from(structure: Structure) -> Self {
        use crate::constants::StructureType::*;

        match structure.structure_type() {
            Container => Self::StructureContainer(structure.unchecked_into()),
            Controller => Self::StructureController(structure.unchecked_into()),
            Extension => Self::StructureExtension(structure.unchecked_into()),
            Extractor => Self::StructureExtractor(structure.unchecked_into()),
            Factory => Self::StructureFactory(structure.unchecked_into()),
            InvaderCore => Self::StructureInvaderCore(structure.unchecked_into()),
            KeeperLair => Self::StructureKeeperLair(structure.unchecked_into()),
            Lab => Self::StructureLab(structure.unchecked_into()),
            Link => Self::StructureLink(structure.unchecked_into()),
            Nuker => Self::StructureNuker(structure.unchecked_into()),
            Observer => Self::StructureObserver(structure.unchecked_into()),
            Portal => Self::StructurePortal(structure.unchecked_into()),
            PowerBank => Self::StructurePowerBank(structure.unchecked_into()),
            PowerSpawn => Self::StructurePowerSpawn(structure.unchecked_into()),
            Rampart => Self::StructureRampart(structure.unchecked_into()),
            Road => Self::StructureRoad(structure.unchecked_into()),
            Spawn => Self::StructureSpawn(structure.unchecked_into()),
            Storage => Self::StructureStorage(structure.unchecked_into()),
            Terminal => Self::StructureTerminal(structure.unchecked_into()),
            Tower => Self::StructureTower(structure.unchecked_into()),
            Wall => Self::StructureWall(structure.unchecked_into()),
            _ => panic!("unknown structure type for conversion into enum"),
        }
    }
}

impl StructureObject {
    pub fn as_has_store(&self) -> Option<&dyn HasStore> {
        match self {
            Self::StructureSpawn(s) => Some(s),
            Self::StructureExtension(s) => Some(s),
            Self::StructureRoad(_) => None,
            Self::StructureWall(_) => None,
            Self::StructureRampart(_) => None,
            Self::StructureKeeperLair(_) => None,
            Self::StructurePortal(_) => None,
            Self::StructureController(_) => None,
            Self::StructureLink(s) => Some(s),
            Self::StructureStorage(s) => Some(s),
            Self::StructureTower(s) => Some(s),
            Self::StructureObserver(_) => None,
            Self::StructurePowerBank(_) => None,
            Self::StructurePowerSpawn(s) => Some(s),
            Self::StructureExtractor(_) => None,
            Self::StructureLab(s) => Some(s),
            Self::StructureTerminal(s) => Some(s),
            Self::StructureContainer(s) => Some(s),
            Self::StructureNuker(s) => Some(s),
            Self::StructureFactory(s) => Some(s),
            Self::StructureInvaderCore(_) => None,
        }
    }
}

impl StructureObject {
    pub fn as_transferable(&self) -> Option<&dyn Transferable> {
        match self {
            Self::StructureSpawn(s) => Some(s),
            Self::StructureExtension(s) => Some(s),
            Self::StructureRoad(_) => None,
            Self::StructureWall(_) => None,
            Self::StructureRampart(_) => None,
            Self::StructureKeeperLair(_) => None,
            Self::StructurePortal(_) => None,
            Self::StructureController(_) => None,
            Self::StructureLink(s) => Some(s),
            Self::StructureStorage(s) => Some(s),
            Self::StructureTower(s) => Some(s),
            Self::StructureObserver(_) => None,
            Self::StructurePowerBank(_) => None,
            Self::StructurePowerSpawn(s) => Some(s),
            Self::StructureExtractor(_) => None,
            Self::StructureLab(s) => Some(s),
            Self::StructureTerminal(s) => Some(s),
            Self::StructureContainer(s) => Some(s),
            Self::StructureNuker(s) => Some(s),
            Self::StructureFactory(s) => Some(s),
            Self::StructureInvaderCore(_) => None,
        }
    }
}

impl StructureObject {
    pub fn as_withdrawable(&self) -> Option<&dyn Withdrawable> {
        match self {
            Self::StructureSpawn(s) => Some(s),
            Self::StructureExtension(s) => Some(s),
            Self::StructureRoad(_) => None,
            Self::StructureWall(_) => None,
            Self::StructureRampart(_) => None,
            Self::StructureKeeperLair(_) => None,
            Self::StructurePortal(_) => None,
            Self::StructureController(_) => None,
            Self::StructureLink(s) => Some(s),
            Self::StructureStorage(s) => Some(s),
            Self::StructureTower(s) => Some(s),
            Self::StructureObserver(_) => None,
            Self::StructurePowerBank(_) => None,
            Self::StructurePowerSpawn(s) => Some(s),
            Self::StructureExtractor(_) => None,
            Self::StructureLab(s) => Some(s),
            Self::StructureTerminal(s) => Some(s),
            Self::StructureContainer(s) => Some(s),
            Self::StructureNuker(_) => None,
            Self::StructureFactory(s) => Some(s),
            Self::StructureInvaderCore(_) => None,
        }
    }
}

impl StructureObject {
    pub fn as_attackable(&self) -> Option<&dyn Attackable> {
        match self {
            Self::StructureSpawn(s) => Some(s),
            Self::StructureExtension(s) => Some(s),
            Self::StructureRoad(s) => Some(s),
            Self::StructureWall(s) => Some(s),
            Self::StructureRampart(s) => Some(s),
            Self::StructureKeeperLair(s) => Some(s),
            Self::StructurePortal(_) => None,
            Self::StructureController(_) => None,
            Self::StructureLink(s) => Some(s),
            Self::StructureStorage(s) => Some(s),
            Self::StructureTower(s) => Some(s),
            Self::StructureObserver(s) => Some(s),
            Self::StructurePowerBank(s) => Some(s),
            Self::StructurePowerSpawn(s) => Some(s),
            Self::StructureExtractor(s) => Some(s),
            Self::StructureLab(s) => Some(s),
            Self::StructureTerminal(s) => Some(s),
            Self::StructureContainer(s) => Some(s),
            Self::StructureNuker(s) => Some(s),
            Self::StructureFactory(s) => Some(s),
            Self::StructureInvaderCore(s) => Some(s),
        }
    }
}