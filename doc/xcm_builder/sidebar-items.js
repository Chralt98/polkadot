window.SIDEBAR_ITEMS = {"enum":[["DispatchBlobError",""],["HaulBlobError",""],["MintLocation","The location which is allowed to mint a particular asset."]],"mod":[["test_utils",""]],"struct":[["Account32Hash",""],["AccountId32Aliases","Extracts the `AccountId32` from the passed `location` if the network matches."],["AccountKey20Aliases",""],["AllowExplicitUnpaidExecutionFrom","Allows execution from any origin that is contained in `T` (i.e. `T::Contains(origin)`) if the message begins with the instruction `UnpaidExecution`."],["AllowKnownQueryResponses","Allows only messages if the generic `ResponseHandler` expects them via `expecting_response`."],["AllowSubscriptionsFrom","Allows execution from `origin` if it is just a straight `SubscribeVersion` or `UnsubscribeVersion` instruction."],["AllowTopLevelPaidExecutionFrom","Allows execution from `origin` if it is contained in `T` (i.e. `T::Contains(origin)`) taking payments into account."],["AllowUnpaidExecutionFrom","Allows execution from any origin that is contained in `T` (i.e. `T::Contains(origin)`)."],["AsPrefixedGeneralIndex","Converter struct implementing `AssetIdConversion` converting a numeric asset ID (must be `TryFrom/TryInto<u128>`) into a `GeneralIndex` junction, prefixed by some `MultiLocation` value. The `MultiLocation` value will typically be a `PalletInstance` junction."],["BackingToPlurality","`Convert` implementation to convert from some an origin which implements `Backing` into a corresponding `Plurality` `MultiLocation`."],["BridgeBlobDispatcher",""],["BridgeMessage",""],["Case","Accepts an asset if it is contained in the given `T`’s `Get` implementation."],["ChildParachainAsNative",""],["ChildParachainConvertsVia",""],["ChildSystemParachainAsSuperuser",""],["ConvertedAbstractId",""],["ConvertedConcreteId",""],["CurrencyAdapter","Simple adapter to use a currency as asset transactor. This type can be used as `type AssetTransactor` in `xcm::Config`."],["DualMint","Implementation of `AssetChecking` which subjects a given set of assets `L` to having their teleportations recorded with a `MintLocation::Local` and a second set of assets `R` to having their teleportations recorded with a `MintLocation::NonLocal`."],["EnsureXcmOrigin","`EnsureOrigin` barrier to convert from dispatch origin to XCM origin, if one exists."],["FixedRateOfFungible","Simple fee calculator that requires payment in a single fungible at a fixed rate."],["FixedWeightBounds",""],["FungiblesAdapter",""],["FungiblesMutateAdapter",""],["FungiblesTransferAdapter","`TransactAsset` implementation to convert a `fungibles` implementation to become usable in XCM."],["HaulBlobExporter",""],["IsAbstract","Same as [`IsConcrete`] but for a fungible with abstract location."],["IsChildSystemParachain","Allows a message only if it is from a system-level child parachain."],["IsConcrete","Converts a `MultiAsset` into balance `B` if it is a concrete fungible with an id equal to that given by `T`’s `Get`."],["LocalMint","Implementation of `AssetChecking` which subjects a given set of assets `T` to having their teleportations recorded with a `MintLocation::Local`."],["MatchedConvertedConcreteId",""],["Matcher","Struct created from calling `fn matcher()` on a mutable slice of `Instruction`s."],["NativeAsset","Accepts an asset iff it is a native asset."],["NetworkExportTable",""],["NoChecking","Implementation of `AssetChecking` which subjects no assets to having their teleportations recorded."],["NonFungiblesAdapter",""],["NonFungiblesMutateAdapter",""],["NonFungiblesTransferAdapter",""],["NonLocalMint","Implementation of `AssetChecking` which subjects a given set of assets `T` to having their teleportations recorded with a `MintLocation::NonLocal`."],["OriginToPluralityVoice","`Convert` implementation to convert from an origin which passes the check of an `EnsureOrigin` into a voice of a given pluralistic `Body`."],["ParentAsSuperuser",""],["ParentIsPreset","A [`MultiLocation`] consisting of a single `Parent` [`Junction`] will be converted to the parent `AccountId`."],["RelayChainAsNative",""],["RespectSuspension","Barrier condition that allows for a `SuspensionChecker` that controls whether or not the XCM executor will be suspended from executing the given XCM."],["SiblingParachainAsNative",""],["SiblingParachainConvertsVia",""],["SiblingSystemParachainAsSuperuser",""],["SignedAccountId32AsNative",""],["SignedAccountKey20AsNative",""],["SignedToAccountId32","`Convert` implementation to convert from some a `Signed` (system) `Origin` into an `AccountId32`."],["SovereignPaidRemoteExporter","Implementation of `SendXcm` which wraps the message inside an `ExportMessage` instruction and sends it to a destination known to be able to handle it."],["SovereignSignedViaLocation","Sovereign accounts use the system’s `Signed` origin with an account ID derived from the `LocationConverter`."],["TakeWeightCredit","Execution barrier that just takes `max_weight` from `weight_credit`."],["UnpaidLocalExporter","Implementation of `SendXcm` which uses the given `ExportXcm` implementation in order to forward the message over a bridge."],["UnpaidRemoteExporter","Implementation of `SendXcm` which wraps the message inside an `ExportMessage` instruction and sends it to a destination known to be able to handle it."],["UsingComponents","Weight trader which uses the configured `WeightToFee` to set the right price for weight and then places any weight bought into the right account."],["WeightInfoBounds",""],["WithComputedOrigin","A derivative barrier, which scans the first `MaxPrefixes` instructions for origin-alterers and then evaluates `should_execute` of the `InnerBarrier` based on the remaining instructions and the newly computed origin."]],"trait":[["AssetChecking","Simple trait to indicate whether an asset is subject to having its teleportation into and out of this chain recorded and if so in what `MintLocation`."],["CreateMatcher","Creates an instruction matcher from an XCM. Since XCM versions differ, we need to make a trait here to unify the interfaces among them."],["DispatchBlob",""],["ExporterFor",""],["HaulBlob",""],["MatchXcm","API that allows to pattern-match against anything that is contained within an XCM."],["TakeRevenue","Function trait for handling some revenue. Similar to a negative imbalance (credit) handler, but for a `MultiAsset`. Sensible implementations will deposit the asset in some known treasury or block-author account."]],"type":[["ConvertedAbstractAssetId",""],["ConvertedConcreteAssetId",""]]};