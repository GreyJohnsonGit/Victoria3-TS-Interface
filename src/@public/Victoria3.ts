import { AppConfig, loadConfig } from '@lib/AppConfig';
import { SafeMap } from '@lib/SafeMap';
import { IStateRegion, StateRegionInterpreter } from '@models/StateRegion';
import { StateRegionName } from '@models/StateRegionName';
import { IInterpreter } from 'interpreter/Interpreter';
import { VTI } from 'interpreter/VTI';

export class Victoria3 {
  public get StateRegions() {
    if (!this._stateRegions) {
      this._stateRegions = new SafeMap(this._vti.encodeFolder<IStateRegion>(
        this._stateRegionInterpreter.path(this._config),
        this._stateRegionInterpreter
      ).map(stateRegion => [stateRegion.get('name'), stateRegion]));
    }
    return this._stateRegions;
  }

  public save() {
    this._vti.decodeToFolder<IStateRegion>(
      Array.from(this.StateRegions.values()),
      this._stateRegionInterpreter,
      this._config
    );
  }

  private readonly _stateRegionInterpreter: IInterpreter<IStateRegion>;
  private _stateRegions: SafeMap<StateRegionName, IStateRegion> | undefined;

  private readonly _vti: VTI;
  private readonly _config: AppConfig;

  constructor(
    stateRegionInterpreter: IInterpreter<IStateRegion> = new StateRegionInterpreter(),
    config: AppConfig = loadConfig()
  ) {
    this._vti = new VTI();
    this._stateRegionInterpreter = stateRegionInterpreter;
    this._config = config;
  }
}