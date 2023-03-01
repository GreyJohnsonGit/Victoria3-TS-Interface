import { x } from '@models/ProvinceId';
import { Victoria3 } from '@public/Victoria3';

const v3 = new Victoria3();

v3.StateRegions.get('STATE_YUKON_TERRITORY').set('name', 'STATE_YUKON_TERRITORY');
v3.StateRegions.get('STATE_YUKON_TERRITORY').setOptional('port', x('00','F2','E3'));
v3.StateRegions.get('STATE_ABRUZZO').setOptional('traits', ['state_trait_nile_river', 'state_trait_zambezi_river']);

v3.save();