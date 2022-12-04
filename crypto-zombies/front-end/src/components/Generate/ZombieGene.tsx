import { useRecoilState, useRecoilValue } from 'recoil';
import { InfomationZombie, NameZombie } from '../../recoil/zombie-detail/zombieDetail';

function ZombieGene() {
  const [textNameZombie, setTextNameZombie] = useRecoilState(NameZombie);
  const infomationZombie = useRecoilValue(InfomationZombie);

  return (
    <div className="text-base mt-12 pt-6 px-3">
      <div className="w-[100%] mt-12 mb-4 pt-12 px-3 flex flex-col justify-evenly  h-[60%]">
        <div>
          <p>
            Head Gene : <span>{infomationZombie.headChoice}</span>
          </p>
        </div>
        <div>
          <p>
            Eye Gene : <span>{infomationZombie.eyeChoice}</span>
          </p>
        </div>
        <div>
          <p>
            Shirt Gene : <span>{infomationZombie.shirtChoice}</span>
          </p>
        </div>
        <div>
          <p>
            Skin Color Gene : <span>{infomationZombie.skinColorChoice}</span>
          </p>
        </div>
        <div>
          <p>
            Eye Color Gene : <span>{infomationZombie.eyeColorChoice}</span>
          </p>
        </div>
        <div>
          <p>
            Clothes Color Gene : <span>{infomationZombie.clothesColorChoice}</span>
          </p>
        </div>
      </div>
      {/* enter name */}
      <div className="mt-8px">
        <input
          type="text"
          name=""
          id=""
          placeholder="Enter name here"
          className="h-10 w-56 rounded-sm px-2 py-3 bg-[#434857]"
          onChange={(e) => {
            setTextNameZombie(e.target.value);
          }}
        />
      </div>
    </div>
  );
}

export default ZombieGene;
