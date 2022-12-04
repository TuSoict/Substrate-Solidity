import { atom } from 'recoil';
import { ZombieDetail } from '../../model/mintZombie';

const initInfomationZombie: ZombieDetail = {
  headChoice: 1,
  eyeChoice: 1,
  shirtChoice: 1,
  skinColorChoice: 0,
  eyeColorChoice: 0,
  clothesColorChoice: 0,
  zombieName: null,
  zombieDescription: 'A Level 1 CryptoZombie',
};
export const InfomationZombie = atom({
  key: 'ZOMBIE_DETAIL',
  default: initInfomationZombie,
});

export const NameZombie = atom({
  key: 'NAME_ZOMBIE',
  default: '',
});
