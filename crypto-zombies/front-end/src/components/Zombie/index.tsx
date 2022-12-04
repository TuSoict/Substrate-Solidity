import { useEffect } from 'react';
import { useRecoilState, useRecoilValue } from 'recoil';
import Web3 from 'web3';
import {
  head1,
  head2,
  head3,
  head4,
  head5,
  head6,
  head7,
  head8,
  eyes1,
  eyes2,
  eyes3,
  eyes4,
  eyes5,
  eyes6,
  eyes7,
  eyes8,
  eyes9,
  eyes10,
  eyes11,
  shirt1,
  shirt2,
  shirt3,
  shirt4,
  shirt5,
  shirt6,
  leftFeet,
  leftHand,
  leftHand2,
  leftThigh,
  leftLeg,
  rightFeet,
  rightHand,
  rightHand2,
  rightThigh,
  rightLeg,
  body,
  mouth,
  duoi,
  hand3,
} from '../../assets/image';
import { InfomationZombie, NameZombie } from '../../recoil/zombie-detail/zombieDetail';
interface props {
  Style?: string;
  gen: string;
}
const Zombie: React.FC<props> = ({ Style, gen }) => {
  const infomationZombie = useRecoilValue(InfomationZombie);

  return (
    <div className={`top-[50px] flex justify-center flex-col items-center ${Style}`}>
      {/* head */}
      <div className="relative z-20">
        <img
          className="w-[209px]"
          src={head1}
          style={{ filter: `hue-rotate(${infomationZombie && infomationZombie.skinColorChoice}deg)` }}
          alt="zombie-img"
        />
        <img
          className="w-[97px] h-[62px] absolute top-[90px] left-[74px]"
          src={eyes1}
          style={{ filter: `hue-rotate(${infomationZombie && infomationZombie.eyeColorChoice}deg)` }}
          alt="zombie-img"
        />
        <img className="w-[44px] h-[35px] absolute top-[143px] left-[103px]" src={mouth} alt="zombie-img" />
      </div>
      <div className="flex flex-col absolute left-[24px] top-[132px] z-10">
        {/* body */}
        <div className="relative z-40">
          <img className="w-[97px] absolute z-10" src={body} alt="zombie-img" />
          <img
            className="w-[97px] absolute z-20"
            src={shirt1}
            style={{ filter: `hue-rotate(${infomationZombie && infomationZombie.clothesColorChoice}deg)` }}
            alt="zombie-img"
          />
          {/* Left hand */}
          <div className="absolute left-[22px] top-[16px] z-30">
            <img
              className="w-[45px] h-[54px]"
              src={hand3}
              style={{ filter: `hue-rotate(${infomationZombie && infomationZombie.skinColorChoice}deg)` }}
              alt="zombie-img"
            />
            <img
              className="w-[30px] h-[16px] absolute top-[32px] left-[20px]"
              src={leftHand2}
              style={{ filter: `hue-rotate(${infomationZombie && infomationZombie.skinColorChoice}deg)` }}
              alt="zombie-img"
            />
            <img
              className="w-[30px] h-[21px] absolute top-[25px] left-[36px]"
              src={leftHand}
              style={{ filter: `hue-rotate(${infomationZombie && infomationZombie.skinColorChoice}deg)` }}
              alt="zombie-img"
            />
          </div>
          {/* Right hand */}
          <div className="absolute left-[50px] top-[18px]">
            <img
              className="w-[45px] h-[54px]  "
              src={hand3}
              style={{ filter: `hue-rotate(${infomationZombie && infomationZombie.skinColorChoice}deg)` }}
              alt="zombie-img"
            />
            <img
              className="w-[30px] h-[16px] absolute top-[30px] left-[18px]"
              src={rightHand2}
              style={{ filter: `hue-rotate(${infomationZombie && infomationZombie.skinColorChoice}deg)` }}
              alt="zombie-img"
            />
            <img
              className="w-[30px] h-[21px] absolute top-[23px] left-[34px]"
              src={rightHand}
              style={{ filter: `hue-rotate(${infomationZombie && infomationZombie.skinColorChoice}deg)` }}
              alt="zombie-img"
            />
          </div>
        </div>
        {/* Chân */}
        <div className="mt-[52px] ml-2">
          {/* Left */}
          <div className="flex relative">
            <img
              className="w-[45px] h-[54px] absolute z-20"
              src={leftThigh}
              style={{ filter: `hue-rotate(${infomationZombie && infomationZombie.skinColorChoice}deg)` }}
              alt="zombie-img"
            />
            <img
              className="w-[30px] h-[36px] absolute top-[38px] left-[7px] z-10"
              src={leftLeg}
              style={{ filter: `hue-rotate(${infomationZombie && infomationZombie.skinColorChoice}deg)` }}
              alt="zombie-img"
            />
            <img
              className="w-[30px] h-[20px] absolute top-[62px] left-[9px]"
              src={leftFeet}
              style={{ filter: `hue-rotate(${infomationZombie && infomationZombie.skinColorChoice}deg)` }}
              alt="zombie-img"
            />
          </div>
          {/* Right */}
          <div className="flex relative ml-6">
            <img
              className="w-[45px] h-[54px] z-20"
              src={rightThigh}
              style={{ filter: `hue-rotate(${infomationZombie && infomationZombie.skinColorChoice}deg)` }}
              alt="zombie-img"
            />
            <img
              className="w-[25px] h-[30px] absolute top-[43px] left-[15px] z-10"
              src={rightLeg}
              style={{ filter: `hue-rotate(${infomationZombie && infomationZombie.skinColorChoice}deg)` }}
              alt="zombie-img"
            />
            <img
              className="w-[25px] h-[16px] absolute top-[63px] left-[16px]"
              src={rightFeet}
              style={{ filter: `hue-rotate(${infomationZombie && infomationZombie.skinColorChoice}deg)` }}
              alt="zombie-img"
            />
          </div>
        </div>
      </div>
    </div>
  );
};

export default Zombie;
